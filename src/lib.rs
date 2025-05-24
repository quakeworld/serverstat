#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! # serverstat
//! Get information from QuakeWorld servers

// internal
mod common;
mod game_server;
mod generic_server;
mod proxy_server;
mod qtv_server;

// public api
pub use common::client_slots::ClientSlots;
pub use common::geo::{Coords, GeoInfo, GeoInfoBuilder};
pub use common::server_type::ServerType;
pub use common::software_type::SoftwareType;

pub use generic_server::qtv_stream::QtvStream;

pub use game_server::player::Player;
pub use game_server::server::GameServer;
pub use game_server::spectator::Spectator;
pub use game_server::team::Team;

pub use proxy_server::client::ProxyClient;
pub use proxy_server::server::ProxyServer;
pub use proxy_server::settings::ProxySettings;

pub use qtv_server::client::QtvClient;
pub use qtv_server::server::QtvServer;
pub use qtv_server::settings::QtvSettings;
