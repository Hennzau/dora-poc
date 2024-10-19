pub mod prelude {
    pub use dpoc_core::{address::DaemonAddress, Application, DaemonLabel};
    pub use dpoc_daemon::{
        queries::{DaemonQuery, DaemonReply},
        Daemon,
    };
    pub use dpoc_parser::{parse_application, read_and_parse_application};
}
