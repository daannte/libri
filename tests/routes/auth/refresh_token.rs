use axum::{
    body::Bytes,
    http::{self, Method, Request},
};
use insta::{assert_json_snapshot, assert_snapshot};
use shiori_test::{test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn refresh_token() {
    let (_, _, user) = TestApp::init_with_user().await;

    let response = user.post::<()>("/api/v1/auth/refresh-token", "{}").await;

    assert_snapshot!(response.status(), @"204 No Content");
    assert!(response.body().is_empty());

    let headers = response.headers();

    let cookies: Vec<_> = headers
        .get_all(http::header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .collect();

    let access_cookie = cookies
        .iter()
        .find(|c| c.starts_with("access_token="))
        .unwrap();
    let refresh_cookie = cookies
        .iter()
        .find(|c| c.starts_with("refresh_token="))
        .unwrap();

    let access_value = access_cookie
        .split(';')
        .next()
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap();

    let refresh_value = refresh_cookie
        .split(';')
        .next()
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap();

    assert_ne!(refresh_value, user.tokens().refresh_token.token);
    assert_ne!(access_value, user.tokens().access_token.token);

    assert!(!access_value.is_empty());
    assert!(!refresh_value.is_empty());

    assert!(access_cookie.contains("HttpOnly"));
    assert!(refresh_cookie.contains("HttpOnly"));

    let response = user.post::<()>("/api/v1/auth/refresh-token", "{}").await;

    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"invalid refresh token\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn invalid_refresh_token() {
    let (_, _, user) = TestApp::init_with_user().await;

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/auth/refresh-token")
        .header(http::header::COOKIE, "refresh_token=fake.jwt.token")
        .body(Bytes::new())
        .unwrap();

    let response = user.run::<()>(request).await;

    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"invalid refresh token\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn access_token_works_after_refresh() {
    let (_, _, user) = TestApp::init_with_user().await;

    let response = user.post::<()>("/api/v1/auth/refresh-token", "{}").await;

    assert_snapshot!(response.status(), @"204 No Content");
    assert!(response.body().is_empty());

    let cookies: Vec<_> = response
        .headers()
        .get_all(http::header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .collect();

    let access_cookie = cookies
        .iter()
        .find(|c| c.starts_with("access_token="))
        .unwrap();

    let access_token = access_cookie.split(';').next().unwrap();

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/auth/me")
        .header(http::header::COOKIE, access_token)
        .body(Bytes::new())
        .unwrap();

    let response = user.run::<()>(request).await;

    assert_snapshot!(response.status(), @"200 OK");
    assert_json_snapshot!(response.json(), {
        ".created_at" => "[datetime]"
    });
}

#[tokio::test(flavor = "multi_thread")]
async fn fail_after_logout() {
    let (_, _, user) = TestApp::init_with_user().await;

    let response = user.post::<()>("/api/v1/auth/logout", "{}").await;

    assert_snapshot!(response.status(), @"204 No Content");
    assert!(response.body().is_empty());

    let response = user.post::<()>("/api/v1/auth/refresh-token", "{}").await;

    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"invalid refresh token\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn access_token_cannot_refresh() {
    let (_, _, user) = TestApp::init_with_user().await;

    let access = user.tokens().access_token.token.clone();

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/auth/refresh-token")
        .header(http::header::COOKIE, format!("refresh_token={}", access))
        .body(Bytes::new())
        .unwrap();

    let response = user.run::<()>(request).await;

    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"invalid refresh token\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn missing_refresh_token() {
    let (_, _, user) = TestApp::init_with_user().await;

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/auth/refresh-token")
        .body(Bytes::new())
        .unwrap();

    let response = user.run::<()>(request).await;

    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"invalid refresh token\"}""#);
}
