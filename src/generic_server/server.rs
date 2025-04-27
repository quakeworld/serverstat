use super::client::QuakeClient;
use super::geo::GeoInfo;
use super::server_type::ServerType;
use super::software_type::SoftwareType;
use super::stream::QtvStream;
use super::svc_status;
use crate::game_server::server::GameServer;
use crate::qtv::server::QtvServer;
use crate::qtv::svc_qtvusers;
use crate::qwfwd::server::QwfwdServer;
use crate::util::net_extra;
use hostport::HostPort;
pub use quake_serverinfo::Settings;
use std::time::Duration;

#[cfg(feature = "json")]
use serde::{Serialize, Serializer, ser::SerializeStruct};

/// Generic Quake server with common functionality
#[derive(Debug, Clone, PartialEq)]
pub struct QuakeServer {
    pub(crate) server_type: ServerType,
    pub(crate) software_type: SoftwareType,
    pub(crate) address: HostPort,
    pub(crate) ip: String,
    pub(crate) settings: Settings,
    pub(crate) clients: Vec<QuakeClient>,
    pub(crate) qtv_stream: Option<QtvStream>,
    pub(crate) geo: GeoInfo,
}

#[allow(dead_code)]
impl QuakeServer {
    pub fn server_type(&self) -> ServerType {
        self.server_type.clone()
    }

    pub fn software_type(&self) -> SoftwareType {
        self.software_type.clone()
    }

    pub fn address(&self) -> &HostPort {
        &self.address
    }

    pub fn ip(&self) -> &str {
        &self.ip
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn clients(&self) -> &[QuakeClient] {
        &self.clients
    }

    pub fn qtv_stream(&self) -> Option<&QtvStream> {
        self.qtv_stream.as_ref()
    }
    pub fn geo(&self) -> &GeoInfo {
        &self.geo
    }

    pub async fn try_from_address(address: &str, timeout: Duration) -> anyhow::Result<Self> {
        let res = svc_status::status_119(address, timeout).await?;
        let ip = net_extra::resolve_address_to_ip(address).unwrap_or_default();

        let qtv_stream = match res.qtv_stream() {
            Some(qtv_stream) => {
                let qtvusers_res = svc_qtvusers::qtvusers(address, timeout)
                    .await
                    .unwrap_or_default();
                Some(qtv_stream.with_client_names(qtvusers_res.client_names()))
            }
            None => None,
        };

        let address = {
            let address_string = res
                .settings()
                .hostport
                .clone()
                .unwrap_or(address.to_string());
            HostPort::try_from(address_string.as_str())?
        };
        let version = res.settings().version.clone().unwrap_or_default();

        Ok(QuakeServer {
            server_type: ServerType::from_version(&version),
            software_type: SoftwareType::from_version(&version),
            address,
            ip,
            settings: res.settings().clone(),
            clients: res.clients().into(),
            qtv_stream,
            geo: GeoInfo::from(&res.settings().clone()),
        })
    }
}

#[cfg(feature = "json")]
impl Serialize for QuakeServer {
    fn serialize<S>(&self, serializer: S) -> anyhow::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        mod field_count {
            pub const COMMON: usize = 7;
            pub const UTIL_SERVER: usize = 3;
            pub const GAME_SERVER: usize = 7;
        }

        let field_count: usize = field_count::COMMON
            + match self.software_type {
                SoftwareType::Qtv | SoftwareType::Qwfwd => field_count::UTIL_SERVER,
                _ => field_count::GAME_SERVER,
            };

        let mut state = serializer.serialize_struct("QuakeServer", field_count)?;
        state.serialize_field("server_type", &self.server_type)?;
        state.serialize_field("software_type", &self.software_type)?;
        state.serialize_field("host", &self.address.host())?;
        state.serialize_field("ip", &self.ip())?;
        state.serialize_field("port", &self.address.port())?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("geo", &self.geo)?;

        if self.software_type == SoftwareType::Qtv {
            let qtv = QtvServer::from(self);
            state.serialize_field("settings", qtv.settings())?;
            state.serialize_field("client_slots", &qtv.client_slots())?;
            state.serialize_field("clients", qtv.clients())?;
        } else if self.software_type == SoftwareType::Qwfwd {
            let qwfwd = QwfwdServer::from(self);
            state.serialize_field("settings", qwfwd.settings())?;
            state.serialize_field("client_slots", &qwfwd.client_slots())?;
            state.serialize_field("clients", qwfwd.clients())?;
        } else {
            let server = GameServer::from(self);
            state.serialize_field("settings", server.settings())?;
            state.serialize_field("player_slots", &server.player_slots())?;
            state.serialize_field("spectator_slots", &server.spectator_slots())?;
            state.serialize_field("teams", server.teams())?;
            state.serialize_field("players", server.players())?;
            state.serialize_field("spectators", server.spectators())?;
            state.serialize_field("qtv_stream", &server.qtv_stream())?;
        }
        state.end()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::generic_server::geo::Coords;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_try_from_address() -> Result<()> {
        // invalid address
        assert!(
            QuakeServer::try_from_address("foo.bar:666", Duration::from_millis(50))
                .await
                .is_err()
        );

        // valid address
        {
            let timeout = Duration::from_secs_f32(0.5);
            let server = QuakeServer::try_from_address("berlin2.qwsv.net:27500", timeout).await?;

            assert_eq!(server.server_type(), ServerType::GameServer);
            assert_eq!(server.software_type(), SoftwareType::Mvdsv);
            assert_eq!(server.address(), &HostPort::new("berlin2.qwsv.net", 27500)?);
            assert!(!server.ip().is_empty());

            let settings = server.settings().clone();
            assert!(settings.hostname.unwrap().starts_with("berlin2 KTX Server"));

            assert!(server.qtv_stream().is_some());

            assert_eq!(
                server.geo(),
                &GeoInfo {
                    country_code: Some("DE".to_string()),
                    city: Some("Berlin".to_string()),
                    region: Some("Europe".to_string()),
                    country_name: Some("Germany".to_string()),
                    coords: Some(Coords::new(52.5200, 13.4050)),
                }
            );
        }

        Ok(())
    }

    #[test]
    fn test_serialize_quakeserver() -> Result<()> {
        let server = QuakeServer {
            server_type: ServerType::GameServer,
            software_type: SoftwareType::Mvdsv,
            address: HostPort::new("localhost", 27500)?,
            ip: "10.10.10.10".to_string(),
            settings: Settings::default(),
            clients: vec![],
            qtv_stream: None,
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                city: Some("New York".to_string()),
                region: Some("NY".to_string()),
                country_name: Some("United States".to_string()),
                coords: Some(Coords::new(40.7128, -74.0060)),
            },
        };
        assert_eq!(
            serde_json::to_string(&server)?,
            r#"{"server_type":"game_server","software_type":"mvdsv","host":"localhost","ip":"10.10.10.10","port":27500,"address":"localhost:27500","geo":{"country_code":"US","country_name":"United States","city":"New York","region":"NY","coords":{"lat":40.7128,"lng":-74.006}},"settings":{"admin":null,"city":null,"coords":null,"countrycode":null,"deathmatch":null,"epoch":null,"fpd":null,"fraglimit":null,"gamedir":null,"hostname":null,"hostport":null,"ktxmode":null,"ktxver":null,"map":null,"matchtag":null,"maxclients":null,"maxfps":null,"maxspectators":null,"mode":null,"needpass":null,"pm_ktjump":null,"progs":null,"qvm":null,"serverdemo":null,"status":null,"sv_antilag":null,"teamplay":null,"timelimit":null,"version":null,"z_ext":null},"player_slots":{"total":0,"used":0,"free":0},"spectator_slots":{"total":0,"used":0,"free":0},"teams":[],"players":[],"spectators":[],"qtv_stream":null}"#
        );
        Ok(())
    }

    #[test]
    fn test_serialize_qtv() -> Result<()> {
        let server = QuakeServer {
            server_type: ServerType::QtvServer,
            software_type: SoftwareType::Qtv,
            address: HostPort::new("qtv", 28000)?,
            ip: "10.10.10.10".to_string(),
            settings: Settings::default(),
            clients: vec![],
            qtv_stream: None,
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                city: Some("New York".to_string()),
                region: Some("NY".to_string()),
                country_name: Some("United States".to_string()),
                coords: Some(Coords::new(40.7128, -74.0060)),
            },
        };
        assert_eq!(
            serde_json::to_string(&server)?,
            r#"{"server_type":"qtv_server","software_type":"qtv","host":"qtv","ip":"10.10.10.10","port":28000,"address":"qtv:28000","geo":{"country_code":"US","country_name":"United States","city":"New York","region":"NY","coords":{"lat":40.7128,"lng":-74.006}},"settings":{"hostname":"","maxclients":0,"version":""},"client_slots":{"total":0,"used":0,"free":0},"clients":[]}"#
        );
        Ok(())
    }

    #[test]
    fn test_serialize_qwfwd() -> Result<()> {
        let server = QuakeServer {
            server_type: ServerType::ProxyServer,
            software_type: SoftwareType::Qwfwd,
            address: HostPort::new("proxy", 30000)?,
            ip: "10.10.10.10".to_string(),
            settings: Settings::default(),
            clients: vec![],
            qtv_stream: None,
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                city: Some("New York".to_string()),
                region: Some("NY".to_string()),
                country_name: Some("United States".to_string()),
                coords: Some(Coords::new(40.7128, -74.0060)),
            },
        };
        assert_eq!(
            serde_json::to_string(&server)?,
            r#"{"server_type":"proxy_server","software_type":"qwfwd","host":"proxy","ip":"10.10.10.10","port":30000,"address":"proxy:30000","geo":{"country_code":"US","country_name":"United States","city":"New York","region":"NY","coords":{"lat":40.7128,"lng":-74.006}},"settings":{"hostname":"","maxclients":0,"version":"","city":null,"coords":null,"countrycode":null,"hostport":null},"client_slots":{"total":0,"used":0,"free":0},"clients":[]}"#
        );
        Ok(())
    }
}
