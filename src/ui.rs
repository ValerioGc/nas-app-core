//! The built interface of an app, served from its folder for every path outside the API.

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use std::path::{Component, Path};

/// Answers `path` with the file under `ui_dir`, `index.html` for `/`. The API is never a file, and
/// a path that leaves the folder is refused.
pub async fn serve_static(ui_dir: &Path, path: &str) -> Response {
    if path.starts_with("/api/") || path == "/api" {
        return StatusCode::NOT_FOUND.into_response();
    }
    let requested = if path == "/" {
        "index.html"
    } else {
        path.strip_prefix('/').unwrap_or(path)
    };
    let relative = Path::new(requested);
    if !relative
        .components()
        .all(|component| matches!(component, Component::Normal(_)))
    {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let file = ui_dir.join(relative);
    let mime = match file
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
    {
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        _ => "application/octet-stream",
    };
    // Hashed assets are cached for good, every other file is revalidated.
    let cache = if requested.starts_with("assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    };
    match tokio::fs::read(file).await {
        Ok(bytes) => (
            [(header::CONTENT_TYPE, mime), (header::CACHE_CONTROL, cache)],
            bytes,
        )
            .into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(test)]
#[path = "../tests/ui.rs"]
mod tests;
