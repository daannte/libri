use axum::{
    body::Bytes,
    http::{self, Method, Request},
};
use insta::{assert_json_snapshot, assert_snapshot};
use shiori_test::{test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn logout() {
    let (_, _, user) = TestApp::init_with_user().await;

    let response = user.post::<()>("/api/v1/auth/logout", "{}").await;

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

    let access_value = access_cookie.split(';').next().unwrap();
    let refresh_value = refresh_cookie.split(';').next().unwrap();

    assert_eq!(access_value, "access_token=");
    assert!(access_cookie.contains("Max-Age=0"));

    assert_eq!(refresh_value, "refresh_token=");
    assert!(refresh_cookie.contains("Max-Age=0"));
}

#[tokio::test(flavor = "multi_thread")]
async fn invalid_access_token() {
    let (_, _, user) = TestApp::init_with_user().await;

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/auth/logout")
        .header(http::header::COOKIE, "access_token=fake.jwt.token")
        .body(Bytes::new())
        .unwrap();

    let response = user.run::<()>(request).await;

    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.json(), @r#"
    {
      "error": "Invalid access token"
    }
    "#);
}
#[tokio::test(flavor = "multi_thread")]
async fn no_cookies() {
    let (_, _, user) = TestApp::init_with_user().await;

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/auth/logout")
        .body(Bytes::new())
        .unwrap();

    let response = user.run::<()>(request).await;

    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.json(), @r#"
    {
      "error": "Unauthorized"
    }
    "#);
}
