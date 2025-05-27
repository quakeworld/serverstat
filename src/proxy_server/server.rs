use crate::{
    ClientSlots, GenericServer, GeoInfo, ProxyClient, ProxySettings, ServerType, SoftwareType,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a proxy server
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProxyServer {
    software_type: SoftwareType,
    address: String,
    ip: String,
    port: u16,
    settings: ProxySettings,
    clients: Vec<ProxyClient>,
    geo: GeoInfo,
}

impl ProxyServer {
    pub fn server_type(&self) -> ServerType {
        ServerType::ProxyServer
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

    pub fn settings(&self) -> &ProxySettings {
        &self.settings
    }

    pub fn clients(&self) -> impl Iterator<Item = &ProxyClient> {
        self.clients.iter()
    }

    pub fn client_slots(&self) -> ClientSlots {
        let total = self.settings().maxclients();
        let used = self.clients.len() as u32;
        ClientSlots::new(used, total)
    }

    pub fn geo(&self) -> &GeoInfo {
        &self.geo
    }
}

impl From<&GenericServer> for ProxyServer {
    fn from(server: &GenericServer) -> Self {
        Self {
            software_type: server.software_type(),
            address: server.address().to_string(),
            ip: server.ip().to_string(),
            port: server.port(),
            settings: ProxySettings::from(server.settings()),
            clients: server.clients().map(ProxyClient::from).collect(),
            geo: server.geo().clone(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::{Coords, GenericClient, GeoInfo, ServerType, SoftwareType};
    use pretty_assertions::assert_eq;
    use quake_serverinfo::Settings;

    #[test]
    fn test_from_genericserver() {
        let generic = GenericServer {
            server_type: ServerType::ProxyServer,
            software_type: SoftwareType::Qtv,
            address: "10.10.10.10".to_string(),
            ip: "10.10.10.10".to_string(),
            port: 28501,
            settings: Settings {
                hostname: Some("LocalQtv".to_string()),
                maxclients: Some(128),
                ..Default::default()
            },
            clients: vec![GenericClient::default(), GenericClient::default()],
            qtv_stream: None,
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                country_name: Some("United States".to_string()),
                city: Some("New York".to_string()),
                region: Some("North America".to_string()),
                coords: Some(Coords::new(40.7128, -74.0060)),
            },
        };
        let server = ProxyServer::from(&generic);
        assert_eq!(server.server_type(), ServerType::ProxyServer);
        assert_eq!(server.software_type(), SoftwareType::Qtv);
        assert_eq!(server.address(), generic.address());
        assert_eq!(server.ip(), generic.ip());
        assert_eq!(server.port(), generic.port());
        assert_eq!(server.clients().count(), generic.clients().count());
        assert_eq!(server.client_slots(), ClientSlots::new(2, 128));
        assert_eq!(server.geo(), generic.geo());
    }
}
