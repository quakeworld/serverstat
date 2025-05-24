use crate::generic_server::query::ServerInfo;
use crate::{
    ClientSlots, GenericServer, GeoInfo, ProxyClient, ProxySettings, ServerType, SoftwareType,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a proxy server
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProxyServer {
    pub(crate) software_type: SoftwareType,
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) settings: ProxySettings,
    pub(crate) clients: Vec<ProxyClient>,
    pub(crate) geo: GeoInfo,
}

impl ProxyServer {
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
}

impl ServerInfo for ProxyServer {
    fn server_type(&self) -> ServerType {
        ServerType::ProxyServer
    }

    fn software_type(&self) -> SoftwareType {
        self.software_type.clone()
    }

    fn ip(&self) -> &str {
        &self.ip
    }
    fn port(&self) -> u16 {
        self.port
    }
}

impl From<&GenericServer> for ProxyServer {
    fn from(server: &GenericServer) -> Self {
        let settings = ProxySettings::from(server.settings());
        let clients = server.clients().map(ProxyClient::from).collect();
        Self {
            software_type: server.software_type(),
            ip: server.ip().to_string(),
            port: server.port(),
            settings,
            clients,
            geo: server.geo().clone(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::generic_server::query::ServerInfo;
    use crate::{GenericClient, ProxyClient, query_async};
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[tokio::test]
    async fn test_from_genericserver() -> Result<()> {
        let server = query_async("quake.se:30000", Duration::from_secs_f32(0.5)).await?;
        assert_eq!(server.port(), 30000);
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
