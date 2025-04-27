//! Generic QuakeWorld server
pub(crate) mod client;
pub(crate) mod client_slots;
pub(crate) mod geo;
pub(crate) mod server;
pub(crate) mod server_type;
pub(crate) mod software_type;
pub(crate) mod stream;
pub(crate) mod svc_status;

pub use client::QuakeClient;
pub use server::QuakeServer;
pub use server::Settings;
pub use stream::QtvStream;
