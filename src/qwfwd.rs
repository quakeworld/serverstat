//! QWFWD: proxy server
use crate::{
    quake_client::QuakeClient,
    quake_server::{ClientSlots, QuakeServer},
};
use quake_serverinfo::Settings;

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

    pub fn clients(&self) -> &[QwfwdClient] {
        &self.clients
    }
}

impl From<&QuakeServer> for QwfwdServer {
    fn from(server: &QuakeServer) -> Self {
        let settings = QwfwdSettings::from(server.settings());
        let clients = server.clients().iter().map(QwfwdClient::from).collect();
        Self { settings, clients }
    }
}

impl QwfwdServer {
    pub fn client_slots(&self) -> ClientSlots {
        let total = self.settings.maxclients;
        let used = self.clients.len() as u32;
        ClientSlots::new(used, total)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct QwfwdSettings {
    hostname: String,
    maxclients: u32,
    version: String,
    city: Option<String>,
    coords: Option<String>,
    countrycode: Option<String>,
    hostport: Option<String>,
}

impl QwfwdSettings {
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn maxclients(&self) -> u32 {
        self.maxclients
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn city(&self) -> Option<&str> {
        self.city.as_deref()
    }

    pub fn coords(&self) -> Option<&str> {
        self.coords.as_deref()
    }

    pub fn countrycode(&self) -> Option<&str> {
        self.countrycode.as_deref()
    }

    pub fn hostport(&self) -> Option<&str> {
        self.hostport.as_deref()
    }
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
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
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
