use quake_serverinfo::Settings;
use quake_text::bytestr::to_unicode;

use crate::client::QuakeClient;
use crate::server::QuakeServer;
use crate::tokenize;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QtvServer {
    pub settings: QtvSettings,
    pub clients: Vec<QtvClient>,
}

impl From<&QuakeServer> for QtvServer {
    fn from(server: &QuakeServer) -> Self {
        let settings = QtvSettings::from(&server.settings);
        let clients = server.clients.iter().map(QtvClient::from).collect();
        Self { settings, clients }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QtvSettings {
    pub hostname: String,
    pub maxclients: u32,
    pub version: String,
}

impl From<&Settings> for QtvSettings {
    fn from(settings: &Settings) -> Self {
        Self {
            hostname: settings.hostname.clone().unwrap_or_default(),
            maxclients: settings.maxclients.unwrap_or_default() as u32,
            version: settings.version.clone().unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QtvClient {
    pub id: u32,
    pub time: u32,
    pub name: String,
}

impl From<&QuakeClient> for QtvClient {
    fn from(client: &QuakeClient) -> Self {
        Self {
            id: client.id,
            time: client.time,
            name: client.name.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QtvStream {
    pub id: u32,
    pub name: String,
    pub url: String,
    pub client_count: u32,
    pub client_names: Vec<String>,
}

impl TryFrom<&[u8]> for QtvStream {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let parts: Vec<String> = tokenize::tokenize(to_unicode(bytes).as_str());
        let id: u32 = parts[1].parse()?;
        let name: String = parts[2].to_string();
        let url: String = parts[3].to_string();
        let client_count: u32 = parts[4].parse()?;

        Ok(Self {
            id,
            name,
            url,
            client_count,
            client_names: vec![],
        })
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
            QuakeServer::try_from_address("quake.se:28000", Duration::from_secs_f32(0.5)).await?;
        assert_eq!(
            QtvServer::from(&server).settings.hostname,
            "QUAKE.SE KTX Qtv"
        );
        Ok(())
    }
}
