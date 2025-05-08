use super::qtv_stream::QtvStream;
use super::quake_client::QuakeClient;
use super::svc_status;
use crate::common::geo::GeoInfo;
use crate::common::server_type::ServerType;
use crate::common::software_type::SoftwareType;
use crate::server::svc_qtvusers;
use crate::util::net_extra;
use hostport::HostPort;
pub use quake_serverinfo::Settings;
use std::time::Duration;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
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

    pub fn clients(&self) -> impl Iterator<Item = &QuakeClient> {
        self.clients.iter()
    }

    pub fn players(&self) -> impl Iterator<Item = &QuakeClient> {
        self.clients().filter(|client| !client.is_spectator())
    }

    pub fn spectators(&self) -> impl Iterator<Item = &QuakeClient> {
        self.clients().filter(|client| client.is_spectator())
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
                let client_names = qtvusers_res.client_names().as_slice();
                Some(qtv_stream.with_client_names(client_names))
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
            clients: res.clients().cloned().collect(),
            qtv_stream,
            geo: GeoInfo::from(res.settings()),
        })
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::common::geo::Coords;
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
}
