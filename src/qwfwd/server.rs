use crate::common::client_slots::ClientSlots;
use crate::qwfwd::client::QwfwdClient;
use crate::qwfwd::settings::QwfwdSettings;
use crate::server::quake_server::QuakeServer;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct QwfwdServer {
    settings: QwfwdSettings,
    clients: Vec<QwfwdClient>,
}

impl QwfwdServer {
    pub fn settings(&self) -> &QwfwdSettings {
        &self.settings
    }

    pub fn clients(&self) -> impl Iterator<Item = &QwfwdClient> {
        self.clients.iter()
    }
}

impl From<&QuakeServer> for QwfwdServer {
    fn from(server: &QuakeServer) -> Self {
        let settings = QwfwdSettings::from(server.settings());
        let clients = server.clients().map(QwfwdClient::from).collect();
        Self { settings, clients }
    }
}

impl QwfwdServer {
    pub fn client_slots(&self) -> ClientSlots {
        let total = self.settings().maxclients();
        let used = self.clients.len() as u32;
        ClientSlots::new(used, total)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::qwfwd::client::QwfwdClient;
    use crate::qwfwd::server::QwfwdServer;
    use crate::server::quake_client::QuakeClient;
    use crate::server::quake_server::QuakeServer;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[tokio::test]
    async fn test_from_gameserver() -> Result<()> {
        let server =
            QuakeServer::try_from_address("quake.se:30000", Duration::from_secs_f32(0.5)).await?;
        assert_eq!(
            QwfwdServer::from(&server).settings().hostname(),
            "QUAKE.SE KTX QWfwd"
        );
        Ok(())
    }

    #[test]
    fn test_from_quakeclient() {
        assert_eq!(
            QwfwdClient::from(&QuakeClient {
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
            QwfwdClient {
                id: 7,
                name: "XantoM".to_string(),
                time: 15,
            }
        );
    }
}
