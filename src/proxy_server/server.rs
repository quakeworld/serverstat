use crate::generic_server::server::GenericServer;
use crate::{ClientSlots, ProxyClient, ProxySettings};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProxyServer {
    settings: ProxySettings,
    clients: Vec<ProxyClient>,
}

impl ProxyServer {
    pub fn settings(&self) -> &ProxySettings {
        &self.settings
    }

    pub fn clients(&self) -> impl Iterator<Item = &ProxyClient> {
        self.clients.iter()
    }
}

impl From<&GenericServer> for ProxyServer {
    fn from(server: &GenericServer) -> Self {
        let settings = ProxySettings::from(server.settings());
        let clients = server.clients().map(ProxyClient::from).collect();
        Self { settings, clients }
    }
}

impl ProxyServer {
    pub fn client_slots(&self) -> ClientSlots {
        let total = self.settings().maxclients();
        let used = self.clients.len() as u32;
        ClientSlots::new(used, total)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::generic_server::client::GenericClient;
    use crate::generic_server::server::GenericServer;
    use crate::{ProxyClient, ProxyServer};
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[tokio::test]
    async fn test_from_gameserver() -> Result<()> {
        let server =
            GenericServer::try_from_address("quake.se:30000", Duration::from_secs_f32(0.5)).await?;
        assert_eq!(
            ProxyServer::from(&server).settings().hostname(),
            "QUAKE.SE KTX QWfwd"
        );
        Ok(())
    }

    #[test]
    fn test_from_quakeclient() {
        assert_eq!(
            ProxyClient::from(&GenericClient {
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
            ProxyClient {
                id: 7,
                name: "XantoM".to_string(),
                time: 15,
            }
        );
    }
}
