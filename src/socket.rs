//! The socket file of an app, taken over only when nothing listens on it any more.

use std::{io::ErrorKind, os::unix::fs::FileTypeExt, path::Path};
use tokio::net::UnixStream;

/// Removes a socket left by a previous run, and refuses a path that is not a socket or that an
/// app still listens on.
pub async fn prepare_socket(path: &Path) -> Result<(), String> {
    if let Ok(metadata) = tokio::fs::symlink_metadata(path).await {
        if !metadata.file_type().is_socket() {
            return Err(format!("Socket path {} is occupied", path.display()));
        }
        match UnixStream::connect(path).await {
            Ok(_) => return Err(format!("Socket {} is already listening", path.display())),
            Err(error) if error.kind() == ErrorKind::ConnectionRefused => {}
            Err(error) => return Err(format!("Unable to check the socket ({error})")),
        }
        tokio::fs::remove_file(path)
            .await
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "../tests/socket.rs"]
mod tests;
