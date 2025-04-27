#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! # serverstat
//! Get information from QuakeWorld servers

// internal modules
pub(crate) mod util;

// public api
pub mod game_server;
pub mod generic_server;
pub mod qtv;
pub mod qwfwd;

pub use crate::generic_server::client_slots::ClientSlots;
pub use crate::generic_server::geo::{Coords, GeoInfo};
pub use crate::generic_server::server_type::ServerType;
pub use crate::generic_server::software_type::SoftwareType;

pub use hostport::HostPort;
