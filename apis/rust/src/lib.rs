pub mod prelude {
    pub use narr_core::daemon::address::DaemonAddress;
    pub use narr_daemon::{queries::DaemonQuery, queries::DaemonReply, Daemon};
    pub use narr_parser::read_toml_and_parse_to_application;
}
