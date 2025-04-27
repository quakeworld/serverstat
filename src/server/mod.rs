//! Generic server (game server, qtv or qwfwd)
pub(crate) mod qtv_stream;
pub(crate) mod quake_client;
pub(crate) mod quake_server;
pub(crate) mod svc_qtvusers;
pub(crate) mod svc_status;

pub use qtv_stream::QtvStream;
pub use quake_client::QuakeClient;
pub use quake_server::QuakeServer;
pub use quake_server::Settings;
