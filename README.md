# nas-app-core

The socket bootstrap shared by the Rust backends of the NAS applications. Every application listens
on a Unix socket that only the launcher may reach, and serves its built interface for the paths
outside its API. This crate does those three things once.

| Item             | What it does                                                                                     |
| ---------------- | ------------------------------------------------------------------------------------------------ |
| `LauncherOnly`   | Listener for `axum::serve` that admits root, the owner of the socket folder and the app itself  |
| `prepare_socket` | Removes a socket left by a previous run, refuses a path that is not a socket or still listens   |
| `serve_static`   | Serves the files of the interface folder, `index.html` for `/`, never the API or a path outside |

An application depends on it through Git, at a tag.

```toml
nas-app-core = { git = "https://github.com/ValerioGc/nas-app-core", tag = "v0.1.0" }
```

```rust
let listener = UnixListener::bind(&socket)?;
axum::serve(nas_app_core::LauncherOnly::new(listener), app).await?;
```

See [DEVELOPMENT.md](DEVELOPMENT.md) for the checks and the releases.
