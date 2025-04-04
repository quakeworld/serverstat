use crate::client::QuakeClient;
use crate::server::QuakeServer;
use quake_serverinfo::Settings;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QwfwdServer {
    pub settings: QwfwdSettings,
    pub clients: Vec<QwfwdClient>,
}

impl From<&QuakeServer> for QwfwdServer {
    fn from(server: &QuakeServer) -> Self {
        let settings = QwfwdSettings::from(&server.settings);
        let clients = server.clients.iter().map(QwfwdClient::from).collect();
        Self { settings, clients }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QwfwdSettings {
    pub hostname: String,
    pub maxclients: u32,
    pub version: String,
    pub city: Option<String>,
    pub coords: Option<String>,
    pub countrycode: Option<String>,
    pub hostport: Option<String>,
}

impl From<&Settings> for QwfwdSettings {
    fn from(settings: &Settings) -> Self {
        Self {
            hostname: settings.hostname.clone().unwrap_or_default(),
            maxclients: settings.maxclients.unwrap_or_default() as u32,
            version: settings.version.clone().unwrap_or_default(),
            city: settings.city.clone(),
            coords: settings.coords.clone(),
            countrycode: settings.countrycode.clone(),
            hostport: settings.hostport.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QwfwdClient {
    pub id: u32,
    pub time: u32,
    pub name: String,
}

impl From<&QuakeClient> for QwfwdClient {
    fn from(client: &QuakeClient) -> Self {
        Self {
            id: client.id,
            time: client.time,
            name: client.name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[tokio::test]
    async fn test_from_gameserver() -> Result<()> {
        let server =
            QuakeServer::try_from_address("quake.se:30000", Duration::from_secs_f32(0.5)).await?;
        assert_eq!(
            QwfwdServer::from(&server).settings.hostname,
            "QUAKE.SE KTX QWfwd"
        );
        Ok(())
    }
}
