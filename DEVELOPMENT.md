# Developing nas-app-core

The crate holds only what every application backend repeats the same way. Code that one
application shapes on its own, such as its errors, its routes or its configuration, stays in the
application.

## Structure

```text
src/
  lib.rs             # Public items
  launcher_only.rs   # Peer uid check on the app socket
  socket.rs          # Takeover of a stale socket file
  ui.rs              # Static files of the interface
tests/               # Unit tests, included by each module with #[path]
```

## Rules

- Keep `cargo fmt` and `cargo clippy --all-targets -- -D warnings` clean.
- Tests live in `tests/` and are included by their module with `#[cfg(test)] #[path = "..."]`,
  never inside a source file. The package has `autotests = false`.
- No dependency beyond `axum` and `tokio` unless it is asked for. Ask before adding one.
- Comments in English, short, and only for what the code does not say. Never the em dash (U+2014).

## Checks

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Releases

A release is a Git tag `vX.Y.Z` on `main`, with the same version in `Cargo.toml`. The applications
point at the tag, so a new version reaches them only when each one moves its tag and runs
`cargo update -p nas-app-core`.
