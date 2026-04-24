use insta::{assert_json_snapshot, assert_snapshot};
use shiori_database::models::NewMedia;
use shiori_test::{test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn returns_media_by_id() {
    let (app, _, user) = TestApp::init_with_user().await;
    let mut conn = app.db_conn().await;

    app.new_library("library", "/data/books").await;

    let new_media = NewMedia {
        name: "book",
        size: 1,
        path: "book.epub",
        extension: "epub",
        library_id: 1,
        cover_path: None,
        koreader_hash: None,
    };

    let m = new_media.insert(&mut conn).await.unwrap();
    let url = format!("/api/v1/media/{}", m.id);

    let response = user.get::<()>(&url).await;
    assert_snapshot!(response.status(), @"200 OK");
    assert_json_snapshot!(response.json(), {
        ".created_at" => "[datetime]"
    });
}
