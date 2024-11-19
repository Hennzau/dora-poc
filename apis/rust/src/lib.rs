pub mod prelude {
    pub use narr_core::{address::DaemonAddress, Application, DaemonLabel};
    pub use narr_daemon::{
        queries::{DaemonQuery, DaemonReply},
        Daemon,
    };
    pub use narr_parser::{parse_application, read_and_parse_application};
}
