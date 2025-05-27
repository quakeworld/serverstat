#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! # serverstat
//! Query QuakeWorld servers for settings and client information.

// internal
mod common;
mod game_server;
mod generic_server;
mod net;
mod proxy_server;
mod qtv_server;
mod server;

// public api
pub use common::client_slots::ClientSlots;
pub use common::geo::{Coords, GeoInfo};
pub use common::server_type::ServerType;
pub use common::software_type::SoftwareType;

pub use generic_server::client::GenericClient;
pub use generic_server::server::GenericServer;

pub use game_server::player::Player;
pub use game_server::qtv_stream::QtvStream;
pub use game_server::server::GameServer;
pub use game_server::spectator::Spectator;
pub use game_server::team::Team;

pub use proxy_server::client::ProxyClient;
pub use proxy_server::server::ProxyServer;
pub use proxy_server::settings::ProxySettings;

pub use net::query::serverinfo;
pub use server::Server;

pub use qtv_server::client::QtvClient;
pub use qtv_server::server::QtvServer;
pub use qtv_server::settings::QtvSettings;

// public api with async support (optional)
#[cfg(feature = "tokio")]
pub use net::query::serverinfo_async;
