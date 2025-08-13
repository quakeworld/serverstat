#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! # serverstat
//! Query QuakeWorld servers for settings and client information.

// internal
mod game_server;
mod generic_server;
mod geo;
mod net;
mod proxy_server;
mod qtv_server;
mod server;
mod server_type;
mod software_type;

// public api
pub use geo::{Coords, GeoInfo};
pub use server::Server;
pub use server_type::ServerType;
pub use software_type::SoftwareType;

pub use generic_server::client::GenericClient;
pub use generic_server::server::GenericServer;

pub use game_server::player::Player;
pub use game_server::qtv_stream::QtvStream;
pub use game_server::server::GameServer;
pub use game_server::settings::GameServerSettings;
pub use game_server::spectator::Spectator;
pub use game_server::team::Team;

pub use net::query::{QueryError, serverinfo};

pub use proxy_server::client::ProxyClient;
pub use proxy_server::server::ProxyServer;
pub use proxy_server::settings::ProxySettings;

pub use qtv_server::client::QtvClient;
pub use qtv_server::server::QtvServer;
pub use qtv_server::settings::QtvSettings;

// public api with async support (optional)
#[cfg(feature = "tokio")]
pub use net::query::serverinfo_async;
