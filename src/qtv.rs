use quake_serverinfo::Settings;
use quake_text::bytestr::to_unicode;

use crate::client::QuakeClient;
use crate::server::QuakeServer;
use crate::tokenize;

use crate::hostport::Hostport;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct QtvStream {
    pub id: u32,
    pub name: String,
    pub number: u32,
    pub address: Hostport,
    pub client_count: u32,
    pub client_names: Vec<String>,
}

impl QtvStream {
    pub fn url(&self) -> String {
        format!("{}@{}", self.number, self.address)
    }
}

impl TryFrom<&[u8]> for QtvStream {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let parts: Vec<String> = tokenize::tokenize(to_unicode(bytes).as_str());
        let id: u32 = parts[1].parse()?;
        let name = parts[2].to_string();
        let url = parts[3].to_string();
        let (number, address) = match url.split_once('@') {
            Some((number_str, hostport)) => {
                let number = number_str.parse::<u32>().unwrap_or_default();
                (number, hostport.to_string())
            }
            None => (0, url.clone()),
        };
        let client_count: u32 = parts[4].parse()?;
        let address = Hostport::try_from(address.as_str())?;

        Ok(Self {
            id,
            name,
            number,
            address,
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

    #[test]
    fn test_qtv_stream_methods() {
        let stream = QtvStream {
            number: 2,
            address: Hostport {
                host: "dm6.uk".to_string(),
                port: 28000,
            },
            ..Default::default()
        };
        assert_eq!(stream.url(), "2@dm6.uk:28000".to_string());
    }

    #[test]
    fn test_qtv_stream_from_bytes() -> Result<()> {
        assert_eq!(
            QtvStream::try_from(br#"nqtv 1 "dm6.uk Qtv (7)" "7@dm6.uk:28000" 4"#.as_ref())?,
            QtvStream {
                id: 1,
                name: "dm6.uk Qtv (7)".to_string(),
                number: 7,
                address: Hostport {
                    host: "dm6.uk".to_string(),
                    port: 28000,
                },
                client_count: 4,
                client_names: vec![],
            }
        );
        Ok(())
    }

    #[test]
    fn test_from_quakeclient() {
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
