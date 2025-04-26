//! QTV: Server for broadcasting
use quake_serverinfo::Settings;
use quake_text::bytestr::to_unicode;

use crate::quake_client::QuakeClient;
use crate::quake_server::{ClientSlots, QuakeServer};
use crate::tokenize;

use crate::hostport::HostPort;
use serde::Serializer;
use serde::ser::SerializeStruct;

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
        let clients = server.clients().iter().map(QtvClient::from).collect();
        Self { settings, clients }
    }
}

impl QtvServer {
    pub fn settings(&self) -> &QtvSettings {
        &self.settings
    }

    pub fn clients(&self) -> &[QtvClient] {
        &self.clients
    }

    pub fn client_slots(&self) -> ClientSlots {
        let total = self.settings.maxclients;
        let used = self.clients.len() as u32;
        ClientSlots::new(used, total)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct QtvSettings {
    hostname: String,
    maxclients: u32,
    version: String,
}

impl QtvSettings {
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn maxclients(&self) -> u32 {
        self.maxclients
    }

    pub fn version(&self) -> &str {
        &self.version
    }
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
    id: u32,
    time: u32,
    name: String,
}

impl QtvClient {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn time(&self) -> u32 {
        self.time
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
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
#[cfg_attr(feature = "json", derive(Deserialize))]
pub struct QtvStream {
    id: u32,
    name: String,
    number: u32,
    address: HostPort,
    client_count: u32,
    client_names: Vec<String>,
}

impl QtvStream {
    pub fn with_client_names(&self, client_names: &[String]) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            number: self.number,
            address: self.address.clone(),
            client_count: self.client_count,
            client_names: client_names.to_vec(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn address(&self) -> &HostPort {
        &self.address
    }

    pub fn client_count(&self) -> u32 {
        self.client_count
    }

    pub fn client_names(&self) -> &[String] {
        &self.client_names
    }

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
        let address = HostPort::try_from(address.as_str())?;

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

#[cfg(feature = "json")]
impl Serialize for QtvStream {
    fn serialize<S>(&self, serializer: S) -> anyhow::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("QtvStream", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("number", &self.number)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("url", &self.url())?;
        state.serialize_field("client_count", &self.client_count)?;
        state.serialize_field("client_names", &self.client_names)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[tokio::test]
    async fn test_qtvserver_from_gameserver() -> Result<()> {
        let server =
            QuakeServer::try_from_address("quake.se:28000", Duration::from_secs_f32(0.5)).await?;
        assert_eq!(
            QtvServer::from(&server).settings.hostname,
            "QUAKE.SE KTX Qtv"
        );
        Ok(())
    }

    #[test]
    fn test_qtvstream_methods() -> Result<()> {
        let stream = QtvStream {
            number: 2,
            address: HostPort::new("dm6.uk".to_string(), 28000)?,
            ..Default::default()
        };
        assert_eq!(stream.url(), "2@dm6.uk:28000".to_string());
        Ok(())
    }

    #[test]
    fn test_qtvstream_from_bytes() -> Result<()> {
        assert_eq!(
            QtvStream::try_from(br#"nqtv 1 "dm6.uk Qtv (7)" "7@dm6.uk:28000" 4"#.as_ref())?,
            QtvStream {
                id: 1,
                name: "dm6.uk Qtv (7)".to_string(),
                number: 7,
                address: HostPort::new("dm6.uk".to_string(), 28000)?,
                client_count: 4,
                client_names: vec![],
            }
        );
        Ok(())
    }

    #[test]
    fn test_qtvstream_serialize() -> Result<()> {
        let server = QtvStream {
            number: 7,
            address: HostPort::new("dm6.uk".to_string(), 28000)?,
            ..Default::default()
        };
        assert!(serde_json::to_string(&server)?.contains(r#""url":"7@dm6.uk:28000""#));
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
