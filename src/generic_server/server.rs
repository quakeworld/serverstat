use crate::{GenericClient, GeoInfo, QtvStream, ServerType, SoftwareType};
pub use quake_serverinfo::Settings;

#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

/// Represents a server of unknown type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
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
        self.clients()
            .iter()
            .filter(|client| !client.is_spectator())
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
}

#[cfg(feature = "serde")]
impl serde::Serialize for GenericServer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("GenericServer", 11)?;
        state.serialize_field("server_type", &self.server_type())?;
        state.serialize_field("software_type", &self.software_type())?;
        state.serialize_field("address", &self.address())?;
        state.serialize_field("ip", self.ip())?;
        state.serialize_field("port", &self.port())?;
        state.serialize_field("settings", self.settings())?;
        state.serialize_field("client_count", &self.clients().len())?;
        state.serialize_field("client_limit", &self.settings().maxclients)?;
        state.serialize_field("clients", self.clients())?;
        state.serialize_field("qtv_stream", &self.qtv_stream())?;
        state.serialize_field("geo", self.geo())?;
        state.end()
    }
}
