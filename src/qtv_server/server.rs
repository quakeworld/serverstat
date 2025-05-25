use crate::generic_server::query::ServerInfo;
use crate::{ClientSlots, GenericServer, QtvClient, QtvSettings, ServerType, SoftwareType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a QTV server
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QtvServer {
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) settings: QtvSettings,
    pub(crate) clients: Vec<QtvClient>,
}

impl From<&GenericServer> for QtvServer {
    fn from(server: &GenericServer) -> Self {
        let settings = QtvSettings::from(server.settings());
        let clients = server.clients().map(QtvClient::from).collect();
        Self {
            ip: server.ip().to_string(),
            port: server.port(),
            settings,
            clients,
        }
    }
}

impl QtvServer {
    pub fn settings(&self) -> &QtvSettings {
        &self.settings
    }

    pub fn clients(&self) -> impl Iterator<Item = &QtvClient> {
        self.clients.iter()
    }

    pub fn client_slots(&self) -> ClientSlots {
        let total = self.settings().maxclients();
        let used = self.clients.len() as u32;
        ClientSlots::new(used, total)
    }
}

impl ServerInfo for QtvServer {
    fn server_type(&self) -> ServerType {
        ServerType::QtvServer
    }

    fn software_type(&self) -> SoftwareType {
        SoftwareType::Qtv
    }

    fn ip(&self) -> &str {
        &self.ip
    }

    fn port(&self) -> u16 {
        self.port
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::generic_server::query::ServerInfo;
    use crate::{GenericClient, QtvClient, QtvStream, query_async};
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[tokio::test]
    async fn test_qtvserver_from_genericserver() -> Result<()> {
        let server = query_async("quake.se:28000", Duration::from_secs_f32(0.5)).await?;
        assert_eq!(server.port(), 28000);
        Ok(())
    }

    #[test]
    fn test_qtvstream_methods() -> Result<()> {
        let stream = QtvStream {
            number: Some(2),
            address: Some("dm6.uk:28000".to_string()),
            ..Default::default()
        };
        assert_eq!(stream.url(), Some("2@dm6.uk:28000".to_string()));
        Ok(())
    }

    #[test]
    fn test_qtvstream_from_bytes() -> Result<()> {
        assert_eq!(
            QtvStream::try_from(br#"nqtv 1 "dm6.uk Qtv (7)" "7@dm6.uk:28000" 4"#.as_ref())?,
            QtvStream {
                id: 1,
                name: "dm6.uk Qtv (7)".to_string(),
                number: Some(7),
                address: Some("dm6.uk:28000".to_string()),
                client_count: 4,
                client_names: vec![],
            }
        );
        Ok(())
    }

    #[test]
    fn test_qtvclient_from_quakeclient() {
        assert_eq!(
            QtvClient::from(&GenericClient {
                id: 7,
                name: "XantoM".to_string(),
                team: "f0m".to_string(),
                frags: 12,
                ping: 25,
                time: 15,
                top_color: 4,
                bottom_color: 2,
                skin: "XantoM".to_string(),
                auth_cc: "xtm".to_string(),
                is_spectator: false,
                is_bot: false,
            }),
            QtvClient {
                id: 7,
                name: "XantoM".to_string(),
                time: 15,
            }
        );
    }
}
