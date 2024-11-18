pub mod prelude {
    pub use narr_core::address::DaemonAddress;
    pub use narr_daemon::{
        queries::{DaemonQuery, DaemonReply, DataFlowQuery, DataFlowReply},
        Daemon,
    };
}
