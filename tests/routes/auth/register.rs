use insta::{assert_json_snapshot, assert_snapshot};
use shiori_api_types::EncodableUser;
use shiori_test::{test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn register() {
    let (_, anon) = TestApp::init_empty().await;

    let body = serde_json::json!({
        "username": "shinei",
        "password": "supercoolpass"
    });

    let response = anon
        .post::<EncodableUser>("/api/v1/auth/register", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"200 OK");
    assert_json_snapshot!(response.json(), {
        ".created_at" => "[datetime]"
    });
    assert!(
        response
            .json()
            .get("is_server_owner")
            .and_then(|v| v.as_bool())
            .unwrap()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn no_username_in_body() {
    let (_, anon) = TestApp::init_empty().await;

    let body = serde_json::json!({
        "password": "supercoolpass"
    });

    let response = anon
        .post::<()>("/api/v1/auth/register", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"422 Unprocessable Entity");
    assert_json_snapshot!(response.text(), @r#""Failed to deserialize the JSON body into the target type: missing field `username` at line 1 column 28""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn no_password_in_body() {
    let (_, anon) = TestApp::init_empty().await;

    let body = serde_json::json!({
        "username": "shinei"
    });

    let response = anon
        .post::<()>("/api/v1/auth/register", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"422 Unprocessable Entity");
    assert_json_snapshot!(response.text(), @r#""Failed to deserialize the JSON body into the target type: missing field `password` at line 1 column 21""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn password_too_short() {
    let (_, anon) = TestApp::init_empty().await;

    let body = serde_json::json!({
        "username": "shinei",
        "password": "short"
    });

    let response = anon
        .post::<()>("/api/v1/auth/register", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"400 Bad Request");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Password must be at least 8 characters\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn non_owner() {
    let (_, anon) = TestApp::init_empty().await;

    let body = serde_json::json!({
        "username": "shinei",
        "password": "supercoolpass"
    });

    let response = anon
        .post::<EncodableUser>("/api/v1/auth/register", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"200 OK");
    assert!(
        response
            .json()
            .get("is_server_owner")
            .and_then(|v| v.as_bool())
            .unwrap()
    );

    let body = serde_json::json!({
        "username": "shinei2",
        "password": "supercoolpass"
    });

    let response = anon
        .post::<EncodableUser>("/api/v1/auth/register", body.to_string())
        .await;

    assert_snapshot!(response.status(), @"403 Forbidden");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Insufficient Permissions\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn authenticated_non_owner() {
    let (_, anon, _) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "username": "shinei",
        "password": "supercoolpass"
    });

    let response = anon
        .post::<EncodableUser>("/api/v1/auth/register", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"403 Forbidden");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Insufficient Permissions\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn duplicate_username() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "username": "shinei",
        "password": "supercoolpass"
    });

    let response = user
        .post::<EncodableUser>("/api/v1/auth/register", body.to_string())
        .await;

    assert_snapshot!(response.status(), @"409 Conflict");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Username already taken\"}""#);
}
