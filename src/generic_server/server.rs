use crate::{GenericClient, GeoInfo, QtvStream, ServerType, SoftwareType};
pub use quake_serverinfo::Settings;

/// Represents a server of unknown type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericServer {
    pub(crate) server_type: ServerType,
    pub(crate) software_type: SoftwareType,
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

    pub fn address(&self) -> String {
        format!("{}:{}", self.ip(), self.port())
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

    pub fn clients(&self) -> &[GenericClient] {
        &self.clients
    }

    pub fn players(&self) -> impl Iterator<Item = &GenericClient> {
        self.clients().iter().filter(|client| !client.is_spectator())
    }

    pub fn spectators(&self) -> impl Iterator<Item = &GenericClient> {
        self.clients().iter().filter(|client| client.is_spectator())
    }

    pub fn qtv_stream(&self) -> Option<&QtvStream> {
        self.qtv_stream.as_ref()
    }

    pub fn geo(&self) -> &GeoInfo {
        &self.geo
    }

    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }
}
