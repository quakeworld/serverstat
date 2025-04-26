//! # serverstat
//! Get information from QuakeWorld servers

// Private modules - implementation details
pub mod game_server;
pub mod geo;
pub mod hostport;
pub(crate) mod net_extra;
pub mod qtv;
pub mod quake_client;
pub mod quake_server;
pub mod qwfwd;
pub mod server_type;
pub mod software_type;
pub(crate) mod svc_qtvusers;
pub(crate) mod svc_status;
pub mod team;
pub(crate) mod tokenize;
