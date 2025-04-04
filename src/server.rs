use crate::client::QuakeClient;
use crate::qtv::QtvStream;
use crate::{svc_qtvusers, svc_status};

use anyhow::Result;
use quake_serverinfo::Settings;
use std::time::Duration;

use crate::server_type::ServerType;
use crate::software_type::SoftwareType;

#[cfg(feature = "serde")]
use {
    crate::gameserver::GameServer,
    crate::qtv::QtvServer,
    crate::qwfwd::QwfwdServer,
    serde::{Serialize, Serializer, ser::SerializeStruct},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuakeServer {
    pub server_type: ServerType,
    pub software_type: SoftwareType,
    pub address: String,
    pub settings: Settings,
    pub clients: Vec<QuakeClient>,
    pub qtv_stream: Option<QtvStream>,
}

impl QuakeServer {
    pub async fn try_from_address(address: &str, timeout: Duration) -> Result<Self> {
        let mut res = svc_status::status_119(address, timeout).await?;

        res.qtv_stream = match res.qtv_stream {
            Some(qtv_stream) => {
                let res = svc_qtvusers::qtvusers(address, timeout)
                    .await
                    .unwrap_or_default();
                Some(QtvStream {
                    client_names: res.client_names,
                    ..qtv_stream
                })
            }
            None => None,
        };

        let version = res.settings.version.as_deref().unwrap_or("");

        Ok(QuakeServer {
            server_type: ServerType::from_version(version),
            software_type: SoftwareType::from_version(version),
            address: address.to_string(),
            settings: res.settings,
            clients: res.clients,
            qtv_stream: res.qtv_stream,
        })
    }
}

#[cfg(feature = "serde")]
impl Serialize for QuakeServer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let field_count: usize = 2 + match self.software_type {
            SoftwareType::Qtv | SoftwareType::Qwfwd => 2,
            _ => 5,
        };

        let mut state = serializer.serialize_struct("QuakeServer", field_count)?;
        state.serialize_field("server_type", &self.server_type)?;
        state.serialize_field("software_type", &self.software_type)?;
        state.serialize_field("address", &self.address)?;

        if self.software_type == SoftwareType::Qtv {
            let qtv = QtvServer::from(self);
            state.serialize_field("settings", &qtv.settings)?;
            state.serialize_field("clients", &qtv.clients)?;
        } else if self.software_type == SoftwareType::Qwfwd {
            let qwfwd = QwfwdServer::from(self);
            state.serialize_field("settings", &qwfwd.settings)?;
            state.serialize_field("clients", &qwfwd.clients)?;
        } else {
            let server = GameServer::from(self);
            state.serialize_field("settings", &server.settings)?;
            state.serialize_field("teams", &server.teams)?;
            state.serialize_field("players", &server.players)?;
            state.serialize_field("spectators", &server.spectators)?;
            state.serialize_field("qtv_stream", &server.qtv_stream)?;
        }

        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_try_from_address() -> Result<()> {
        assert!(
            QuakeServer::try_from_address("foo.bar:666", Duration::from_millis(50))
                .await
                .is_err()
        );
        assert!(
            QuakeServer::try_from_address("quake.se:28501", Duration::from_secs_f32(0.5))
                .await?
                .settings
                .hostname
                .unwrap()
                .starts_with("QUAKE.SE KTX:28501")
        );
        Ok(())
    }
}
