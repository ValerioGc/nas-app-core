//! What every backend of a NAS application does the same way before its own routes. It admits on
//! its socket only the launcher, takes over a stale socket file, and serves its built interface.

mod launcher_only;
mod socket;
mod ui;

pub use launcher_only::LauncherOnly;
pub use socket::prepare_socket;
pub use ui::serve_static;
