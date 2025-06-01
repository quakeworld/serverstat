use crate::{GenericServer, GeoInfo, ProxyClient, ProxySettings, ServerType, SoftwareType};

#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

/// Represents a proxy server
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ProxyServer {
    software_type: SoftwareType,
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

    pub fn address(&self) -> String {
        format!("{}:{}", self.ip(), self.port())
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

    pub fn clients(&self) -> &[ProxyClient] {
        &self.clients
    }

    pub fn geo(&self) -> &GeoInfo {
        &self.geo
    }

    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }
}

impl From<&GenericServer> for ProxyServer {
    fn from(server: &GenericServer) -> Self {
        Self {
            software_type: server.software_type(),
            ip: server.ip().to_string(),
            port: server.port(),
            settings: ProxySettings::from(server.settings()),
            clients: server.clients().iter().map(ProxyClient::from).collect(),
            geo: server.geo().clone(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ProxyServer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ProxyServer", 10)?;
        state.serialize_field("server_type", &self.server_type())?;
        state.serialize_field("software_type", &self.software_type())?;
        state.serialize_field("address", &self.address())?;
        state.serialize_field("ip", self.ip())?;
        state.serialize_field("port", &self.port())?;
        state.serialize_field("settings", self.settings())?;
        state.serialize_field("client_count", &self.clients().len())?;
        state.serialize_field("client_limit", &self.settings().maxclients())?;
        state.serialize_field("clients", self.clients())?;
        state.serialize_field("geo", self.geo())?;
        state.end()
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
        let proxy = ProxyServer::from(&generic);
        assert_eq!(proxy.server_type(), ServerType::ProxyServer);
        assert_eq!(proxy.software_type(), SoftwareType::Qtv);
        assert_eq!(proxy.address(), generic.address());
        assert_eq!(proxy.ip(), generic.ip());
        assert_eq!(proxy.port(), generic.port());
        assert_eq!(proxy.clients().len(), generic.clients().len());
        assert_eq!(proxy.geo(), generic.geo());
        assert!(!proxy.is_empty());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization() -> anyhow::Result<()> {
        let proxy = ProxyServer {
            software_type: SoftwareType::Qwfwd,
            ip: "10.10.10.10".to_string(),
            port: 28000,
            settings: ProxySettings {
                hostname: "LocalProxy".to_string(),
                maxclients: 128,
                version: "QWFWD 1.0".to_string(),
                ..Default::default()
            },
            clients: vec![ProxyClient {
                id: 1,
                time: 64,
                name: "XantoM".to_string(),
            }],
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                country_name: Some("United States".to_string()),
                city: Some("New York".to_string()),
                region: Some("North America".to_string()),
                coords: Some(Coords::new(40.7128, -74.0060)),
            },
        };

        let proxy_json = r#"{"server_type":"proxy_server","software_type":"qwfwd","address":"10.10.10.10:28000","ip":"10.10.10.10","port":28000,"settings":{"hostname":"LocalProxy","maxclients":128,"version":"QWFWD 1.0","city":null,"coords":null,"countrycode":null,"hostport":null},"client_count":1,"client_limit":128,"clients":[{"id":1,"time":64,"name":"XantoM"}],"geo":{"country_code":"US","country_name":"United States","city":"New York","region":"North America","coords":{"lat":40.7128,"lng":-74.006}}}"#;

        // ensure round-trip serialization
        assert_eq!(serde_json::to_string(&proxy)?, proxy_json);
        assert_eq!(serde_json::from_str::<ProxyServer>(proxy_json)?, proxy);

        Ok(())
    }
}
