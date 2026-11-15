use super::*;
use std::path::PathBuf;
use tokio::net::UnixListener;

fn folder(name: &str) -> PathBuf {
    let folder =
        std::env::temp_dir().join(format!("nas-app-core-socket-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).expect("folder");
    folder
}

#[tokio::test]
async fn a_missing_socket_is_fine_and_a_stale_one_is_removed() {
    let folder = folder("stale");
    let socket = folder.join("app.sock");
    assert!(prepare_socket(&socket).await.is_ok());

    drop(UnixListener::bind(&socket).expect("socket"));
    assert!(socket.exists());
    assert!(prepare_socket(&socket).await.is_ok());
    assert!(!socket.exists());
    std::fs::remove_dir_all(&folder).expect("cleanup");
}

#[tokio::test]
async fn a_file_or_a_socket_still_listening_is_left_alone() {
    let folder = folder("busy");
    let file = folder.join("file.sock");
    std::fs::write(&file, "not a socket").expect("file");
    assert!(prepare_socket(&file)
        .await
        .expect_err("a file")
        .contains("occupied"));

    let socket = folder.join("app.sock");
    let _listener = UnixListener::bind(&socket).expect("socket");
    assert!(prepare_socket(&socket)
        .await
        .expect_err("listening")
        .contains("already listening"));
    assert!(socket.exists());
    std::fs::remove_dir_all(&folder).expect("cleanup");
}
