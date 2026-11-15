# nas-app-core

The socket bootstrap shared by the backends of the NAS applications, one of the projects of
nas-os (see `../CLAUDE.md`). Read [README.md](README.md) and [DEVELOPMENT.md](DEVELOPMENT.md)
before changing anything.

## Before handing work in

Run these and report the real outcome. A change that leaves one of them red is not done.

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

A change here reaches every application. Check that the applications still build and pass their
tests with it before handing it in.
