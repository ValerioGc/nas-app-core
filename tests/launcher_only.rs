use super::*;

#[tokio::test]
async fn answers_the_owner_of_the_folder_of_the_sockets_and_root_only() {
    let folder = std::env::temp_dir().join(format!("nas-app-core-only-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).expect("folder");
    let socket = folder.join("app.sock");
    let mut only = LauncherOnly::new(UnixListener::bind(&socket).expect("socket"));
    let owner = std::fs::metadata(&folder).expect("metadata").uid();
    assert!(only.admits(owner));
    assert!(only.admits(0));
    if owner != 0 && owner != u32::MAX {
        assert!(!only.admits(owner + 1));
    }
    let client = tokio::spawn(UnixStream::connect(socket.clone()));
    let _ = Listener::accept(&mut only).await;
    assert!(client.await.expect("client").is_ok());
    assert!(only.local_addr().is_ok());
    std::fs::remove_dir_all(&folder).expect("cleanup");
}

#[test]
fn a_bare_socket_name_is_in_the_current_folder() {
    assert_eq!(folder_of(Path::new("cli.sock")), PathBuf::from("."));
    assert_eq!(
        folder_of(Path::new("/run/nas-os/apps/app.sock")),
        PathBuf::from("/run/nas-os/apps")
    );
}
