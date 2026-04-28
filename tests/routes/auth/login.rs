use insta::{assert_json_snapshot, assert_snapshot};
use shiori_test::{test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn login() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "username": "shinei",
        "password": "supercoolpass"
    });

    let response = user
        .post::<()>("/api/v1/auth/login", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"200 OK");
    assert_json_snapshot!(response.json(), {
        ".created_at" => "[datetime]"
    });
}

#[tokio::test(flavor = "multi_thread")]
async fn wrong_username() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "username": "no_user",
        "password": "supercoolpass"
    });

    let response = user
        .post::<()>("/api/v1/auth/login", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.json(), @r#"
    {
      "error": "Invalid credentials"
    }
    "#);
}

#[tokio::test(flavor = "multi_thread")]
async fn wrong_password() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "username": "shinei",
        "password": "supercoolpass1"
    });

    let response = user
        .post::<()>("/api/v1/auth/login", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.json(), @r#"
    {
      "error": "Invalid credentials"
    }
    "#);
}

#[tokio::test(flavor = "multi_thread")]
async fn sql_injection_fail() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "username": "' OR 1=1 --",
        "password": "supercoolpass"
    });

    let response = user
        .post::<()>("/api/v1/auth/login", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.json(), @r#"
    {
      "error": "Invalid credentials"
    }
    "#);
}

#[tokio::test(flavor = "multi_thread")]
async fn no_username_in_body() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "password": "supercoolpass"
    });

    let response = user
        .post::<()>("/api/v1/auth/login", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"422 Unprocessable Entity");
    assert_json_snapshot!(response.text(), @r#""Failed to deserialize the JSON body into the target type: missing field `username` at line 1 column 28""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn no_password_in_body() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "username": "shinei",
    });

    let response = user
        .post::<()>("/api/v1/auth/login", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"422 Unprocessable Entity");
    assert_json_snapshot!(response.text(), @r#""Failed to deserialize the JSON body into the target type: missing field `password` at line 1 column 21""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn username_case_sensitivity() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "username": "Shinei",
        "password": "supercoolpass"
    });

    let response = user
        .post::<()>("/api/v1/auth/login", body.to_string())
        .await;
    assert_snapshot!(response.status(), @"200 OK");
    assert_json_snapshot!(response.json(), {
        ".created_at" => "[datetime]"
    });
}
