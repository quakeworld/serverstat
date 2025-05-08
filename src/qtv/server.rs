use crate::common::client_slots::ClientSlots;
use crate::qtv::client::QtvClient;
use crate::qtv::settings::QtvSettings;
use crate::server::quake_server::QuakeServer;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct QtvServer {
    settings: QtvSettings,
    clients: Vec<QtvClient>,
}

impl From<&QuakeServer> for QtvServer {
    fn from(server: &QuakeServer) -> Self {
        let settings = QtvSettings::from(server.settings());
        let clients = server.clients().map(QtvClient::from).collect();
        Self { settings, clients }
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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::qtv::client::QtvClient;
    use crate::qtv::server::QtvServer;
    use crate::server::qtv_stream::QtvStream;
    use crate::server::quake_client::QuakeClient;
    use crate::server::quake_server::QuakeServer;
    use anyhow::Result;
    use hostport::HostPort;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[tokio::test]
    async fn test_qtvserver_from_gameserver() -> Result<()> {
        let server =
            QuakeServer::try_from_address("quake.se:28000", Duration::from_secs_f32(0.5)).await?;
        assert_eq!(
            QtvServer::from(&server).settings().hostname(),
            "QUAKE.SE KTX Qtv"
        );
        Ok(())
    }

    #[test]
    fn test_qtvstream_methods() -> Result<()> {
        let stream = QtvStream {
            number: Some(2),
            address: Some(HostPort::new("dm6.uk", 28000)?),
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
                address: Some(HostPort::new("dm6.uk", 28000)?),
                client_count: 4,
                client_names: vec![],
            }
        );
        Ok(())
    }

    #[test]
    fn test_qtvclient_from_quakeclient() {
        assert_eq!(
            QtvClient::from(&QuakeClient {
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
