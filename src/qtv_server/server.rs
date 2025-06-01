use crate::{GenericServer, QtvClient, QtvSettings, ServerType, SoftwareType};

#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

/// Represents a QTV server
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct QtvServer {
    ip: String,
    port: u16,
    settings: QtvSettings,
    clients: Vec<QtvClient>,
}

impl QtvServer {
    pub fn server_type(&self) -> ServerType {
        ServerType::QtvServer
    }

    pub fn software_type(&self) -> SoftwareType {
        SoftwareType::Qtv
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

    pub fn settings(&self) -> &QtvSettings {
        &self.settings
    }

    pub fn clients(&self) -> &[QtvClient] {
        &self.clients
    }

    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }
}

impl From<&GenericServer> for QtvServer {
    fn from(server: &GenericServer) -> Self {
        Self {
            ip: server.ip().to_string(),
            port: server.port(),
            settings: QtvSettings::from(server.settings()),
            clients: server.clients().iter().map(QtvClient::from).collect(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QtvServer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("QtvServer", 9)?;
        state.serialize_field("server_type", &self.server_type())?;
        state.serialize_field("software_type", &self.software_type())?;
        state.serialize_field("address", &self.address())?;
        state.serialize_field("ip", self.ip())?;
        state.serialize_field("port", &self.port())?;
        state.serialize_field("settings", self.settings())?;
        state.serialize_field("client_count", &self.clients().len())?;
        state.serialize_field("client_limit", &self.settings().maxclients())?;
        state.serialize_field("clients", self.clients())?;
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
            server_type: ServerType::QtvServer,
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
        let qtv = QtvServer::from(&generic);
        assert_eq!(qtv.server_type(), ServerType::QtvServer);
        assert_eq!(qtv.software_type(), SoftwareType::Qtv);
        assert_eq!(qtv.address(), generic.address());
        assert_eq!(qtv.ip(), generic.ip());
        assert_eq!(qtv.port(), generic.port());
        assert_eq!(qtv.clients().len(), generic.clients().len());
        assert!(!qtv.is_empty());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() -> anyhow::Result<()> {
        let qtv = QtvServer {
            ip: "10.10.10.10".to_string(),
            port: 28000,
            settings: QtvSettings {
                hostname: "LocalQTV".to_string(),
                maxclients: 128,
                version: "QTVGO 1.16-dev".to_string(),
            },
            clients: vec![QtvClient {
                id: 1,
                time: 64,
                name: "XantoM".to_string(),
            }],
        };

        let qtv_json = r#"{"server_type":"qtv_server","software_type":"qtv","address":"10.10.10.10:28000","ip":"10.10.10.10","port":28000,"settings":{"hostname":"LocalQTV","maxclients":128,"version":"QTVGO 1.16-dev"},"client_count":1,"client_limit":128,"clients":[{"id":1,"time":64,"name":"XantoM"}]}"#;

        // ensure round-trip serialization
        assert_eq!(serde_json::to_string(&qtv)?, qtv_json);
        assert_eq!(serde_json::from_str::<QtvServer>(qtv_json)?, qtv);

        Ok(())
    }
}
