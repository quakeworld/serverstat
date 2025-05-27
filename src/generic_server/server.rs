use crate::{GenericClient, GeoInfo, QtvStream, ServerType, SoftwareType};
pub use quake_serverinfo::Settings;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a server of unknown type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenericServer {
    pub(crate) server_type: ServerType,
    pub(crate) software_type: SoftwareType,
    pub(crate) address: String,
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) settings: Settings,
    pub(crate) clients: Vec<GenericClient>,
    pub(crate) qtv_stream: Option<QtvStream>,
    pub(crate) geo: GeoInfo,
}

#[allow(dead_code)]
impl GenericServer {
    pub fn server_type(&self) -> ServerType {
        self.server_type.clone()
    }

    pub fn software_type(&self) -> SoftwareType {
        self.software_type.clone()
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn ip(&self) -> &str {
        &self.ip
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn clients(&self) -> impl Iterator<Item = &GenericClient> {
        self.clients.iter()
    }

    pub fn players(&self) -> impl Iterator<Item = &GenericClient> {
        self.clients().filter(|client| !client.is_spectator())
    }

    pub fn spectators(&self) -> impl Iterator<Item = &GenericClient> {
        self.clients().filter(|client| client.is_spectator())
    }

    pub fn qtv_stream(&self) -> Option<&QtvStream> {
        self.qtv_stream.as_ref()
    }
    pub fn geo(&self) -> &GeoInfo {
        &self.geo
    }
}
