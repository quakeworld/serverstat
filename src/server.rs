use anyhow::Result;
use std::time::Duration;

pub use quake_serverinfo::Settings;

use crate::client::QuakeClient;
use crate::geo::GeoInfo;
use crate::hostport::Hostport;
use crate::qtv::QtvStream;
use crate::server_type::ServerType;
use crate::software_type::SoftwareType;
use crate::svc_status;
use crate::{net_extra, svc_qtvusers};

#[cfg(feature = "json")]
use {
    crate::gameserver::GameServer,
    crate::qtv::QtvServer,
    crate::qwfwd::QwfwdServer,
    serde::{Serialize, Serializer, ser::SerializeStruct},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuakeServer {
    pub server_type: ServerType,
    pub software_type: SoftwareType,
    pub address: Hostport,
    pub ip: String,
    pub settings: Settings,
    pub clients: Vec<QuakeClient>,
    pub qtv_stream: Option<QtvStream>,
    pub geo: GeoInfo,
}

impl QuakeServer {
    pub async fn try_from_address(address: &str, timeout: Duration) -> Result<Self> {
        let mut res = svc_status::status_119(address, timeout).await?;
        let ip = net_extra::address_to_ip(address).unwrap_or_default();

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

        let address = {
            let address_string = res.settings.hostport.clone().unwrap_or(address.to_string());
            Hostport::try_from(address_string.as_str())?
        };
        let version = res.settings.version.clone().unwrap_or_default();

        Ok(QuakeServer {
            server_type: ServerType::from_version(&version),
            software_type: SoftwareType::from_version(&version),
            address,
            ip,
            settings: res.settings.clone(),
            clients: res.clients,
            qtv_stream: res.qtv_stream,
            geo: GeoInfo::from(&res.settings),
        })
    }
}

#[cfg(feature = "json")]
impl Serialize for QuakeServer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let field_count: usize = 7 + match self.software_type {
            SoftwareType::Qtv | SoftwareType::Qwfwd => 2,
            _ => 5,
        };

        let mut state = serializer.serialize_struct("QuakeServer", field_count)?;
        state.serialize_field("server_type", &self.server_type)?;
        state.serialize_field("software_type", &self.software_type)?;
        state.serialize_field("host", &self.address.host)?;
        state.serialize_field("ip", &self.ip)?;
        state.serialize_field("port", &self.address.port)?;
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

        state.serialize_field("geo", &self.geo)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::Coordinates;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_try_from_address() -> Result<()> {
        assert!(
            QuakeServer::try_from_address("foo.bar:666", Duration::from_millis(50))
                .await
                .is_err()
        );

        let timeout = Duration::from_secs_f32(0.5);
        let server = QuakeServer::try_from_address("berlin2.qwsv.net:27500", timeout).await?;

        assert!(
            server
                .clone()
                .settings
                .hostname
                .unwrap()
                .starts_with("berlin2 KTX Server")
        );
        assert_eq!(
            server.address,
            Hostport {
                host: "berlin2.qwsv.net".to_string(),
                port: 27500,
            }
        );

        assert_eq!(
            server.geo,
            GeoInfo {
                country_code: Some("DE".to_string()),
                city: Some("Berlin".to_string()),
                region: Some("Europe".to_string()),
                country_name: Some("Germany".to_string()),
                coords: Some(Coordinates {
                    lat: 52.5200,
                    lng: 13.4050,
                }),
            }
        );

        Ok(())
    }

    #[test]
    fn test_serialize_quakeserver() -> Result<()> {
        let server = QuakeServer {
            server_type: ServerType::GameServer,
            software_type: SoftwareType::Mvdsv,
            address: Hostport {
                host: "localhost".to_string(),
                port: 27500,
            },
            ip: "10.10.10.10".to_string(),
            settings: Settings::default(),
            clients: vec![],
            qtv_stream: None,
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                city: Some("New York".to_string()),
                region: Some("NY".to_string()),
                country_name: Some("United States".to_string()),
                coords: Some(Coordinates {
                    lat: 40.7128,
                    lng: -74.0060,
                }),
            },
        };
        assert_eq!(
            serde_json::to_string(&server)?,
            r#"{"server_type":"game_server","software_type":"mvdsv","host":"localhost","ip":"10.10.10.10","port":27500,"address":"localhost:27500","settings":{"admin":null,"city":null,"coords":null,"countrycode":null,"deathmatch":null,"epoch":null,"fpd":null,"fraglimit":null,"gamedir":null,"hostname":null,"hostport":null,"ktxmode":null,"ktxver":null,"map":null,"matchtag":null,"maxclients":null,"maxfps":null,"maxspectators":null,"mode":null,"needpass":null,"pm_ktjump":null,"progs":null,"qvm":null,"serverdemo":null,"status":null,"sv_antilag":null,"teamplay":null,"timelimit":null,"version":null,"z_ext":null},"teams":[],"players":[],"spectators":[],"qtv_stream":null,"geo":{"country_code":"US","country_name":"United States","city":"New York","region":"NY","coords":{"lat":40.7128,"lng":-74.006}}}"#
        );
        Ok(())
    }

    #[test]
    fn test_serialize_qtv() -> Result<()> {
        let server = QuakeServer {
            server_type: ServerType::QtvServer,
            software_type: SoftwareType::Qtv,
            address: Hostport {
                host: "localhost qtv".to_string(),
                port: 28000,
            },
            ip: "10.10.10.10".to_string(),
            settings: Settings::default(),
            clients: vec![],
            qtv_stream: None,
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                city: Some("New York".to_string()),
                region: Some("NY".to_string()),
                country_name: Some("United States".to_string()),
                coords: Some(Coordinates {
                    lat: 40.7128,
                    lng: -74.0060,
                }),
            },
        };
        assert_eq!(
            serde_json::to_string(&server)?,
            r#"{"server_type":"qtv_server","software_type":"qtv","host":"localhost qtv","ip":"10.10.10.10","port":28000,"address":"localhost qtv:28000","settings":{"hostname":"","maxclients":0,"version":""},"clients":[],"geo":{"country_code":"US","country_name":"United States","city":"New York","region":"NY","coords":{"lat":40.7128,"lng":-74.006}}}"#
        );
        Ok(())
    }

    #[test]
    fn test_serialize_qwfwd() -> Result<()> {
        let server = QuakeServer {
            server_type: ServerType::ProxyServer,
            software_type: SoftwareType::Qwfwd,
            address: Hostport {
                host: "localhost proxy".to_string(),
                port: 30000,
            },
            ip: "10.10.10.10".to_string(),
            settings: Settings::default(),
            clients: vec![],
            qtv_stream: None,
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                city: Some("New York".to_string()),
                region: Some("NY".to_string()),
                country_name: Some("United States".to_string()),
                coords: Some(Coordinates {
                    lat: 40.7128,
                    lng: -74.0060,
                }),
            },
        };
        assert_eq!(
            serde_json::to_string(&server)?,
            r#"{"server_type":"proxy_server","software_type":"qwfwd","host":"localhost proxy","ip":"10.10.10.10","port":30000,"address":"localhost proxy:30000","settings":{"hostname":"","maxclients":0,"version":"","city":null,"coords":null,"countrycode":null,"hostport":null},"clients":[],"geo":{"country_code":"US","country_name":"United States","city":"New York","region":"NY","coords":{"lat":40.7128,"lng":-74.006}}}"#
        );
        Ok(())
    }
}
