#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! # serverstat
//! Get information from QuakeWorld servers

// internal modules
pub(crate) mod common;
pub(crate) mod util;

// public api
pub mod game_server;
pub mod qtv;
pub mod qwfwd;
pub mod server;

pub use common::client_slots::ClientSlots;
pub use common::geo::{Coords, GeoInfo, GeoInfoBuilder};
pub use common::server_type::ServerType;
pub use common::software_type::SoftwareType;
