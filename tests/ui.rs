use super::*;
use axum::http::StatusCode;
use std::path::PathBuf;

fn interface() -> PathBuf {
    let folder = std::env::temp_dir().join(format!("nas-app-core-ui-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(folder.join("assets")).expect("folder");
    std::fs::write(folder.join("index.html"), "<main></main>").expect("index");
    std::fs::write(folder.join("assets/app-1a2b.js"), "export {}").expect("asset");
    folder
}

fn header(response: &Response, name: header::HeaderName) -> String {
    response.headers()[name]
        .to_str()
        .expect("header")
        .to_owned()
}

#[tokio::test]
async fn serves_the_interface_and_nothing_outside_it() {
    let folder = interface();

    let index = serve_static(&folder, "/").await;
    assert_eq!(index.status(), StatusCode::OK);
    assert_eq!(
        header(&index, header::CONTENT_TYPE),
        "text/html; charset=utf-8"
    );
    assert_eq!(header(&index, header::CACHE_CONTROL), "no-cache");

    let asset = serve_static(&folder, "/assets/app-1a2b.js").await;
    assert_eq!(asset.status(), StatusCode::OK);
    assert_eq!(
        header(&asset, header::CACHE_CONTROL),
        "public, max-age=31536000, immutable"
    );

    assert_eq!(
        serve_static(&folder, "/api").await.status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        serve_static(&folder, "/api/rules").await.status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        serve_static(&folder, "/missing.css").await.status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        serve_static(&folder, "/../secret").await.status(),
        StatusCode::BAD_REQUEST
    );
    std::fs::remove_dir_all(&folder).expect("cleanup");
}
