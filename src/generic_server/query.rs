use super::net_resolve::resolve_host;
use super::server::GenericServer;
use super::svc_qtvusers::qtvusers;
use super::svc_status::{Status119Response, status_119};
use crate::{GameServer, GeoInfo, ProxyServer, QtvServer, QtvStream, ServerType, SoftwareType};
use anyhow::Result;
use hostport::HostPort;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum Server {
    Proxy(ProxyServer),
    Game(GameServer),
    Qtv(QtvServer),
    Generic(GenericServer),
}

impl ServerInfo for Server {
    fn server_type(&self) -> ServerType {
        match self {
            Server::Proxy(_) => ServerType::ProxyServer,
            Server::Game(_) => ServerType::GameServer,
            Server::Qtv(_) => ServerType::QtvServer,
            Server::Generic(_) => ServerType::Unknown,
        }
    }

    fn software_type(&self) -> SoftwareType {
        match self {
            Server::Proxy(s) => s.software_type(),
            Server::Game(s) => s.software_type(),
            Server::Qtv(s) => s.software_type(),
            Server::Generic(s) => s.software_type(),
        }
    }

    fn ip(&self) -> &str {
        match self {
            Server::Game(s) => s.ip(),
            Server::Proxy(s) => s.ip(),
            Server::Qtv(s) => s.ip(),
            Server::Generic(s) => s.ip(),
        }
    }

    fn port(&self) -> u16 {
        match self {
            Server::Game(s) => s.port(),
            Server::Proxy(s) => s.port(),
            Server::Qtv(s) => s.port(),
            Server::Generic(s) => s.port(),
        }
    }

    fn address(&self) -> String {
        match self {
            Server::Game(s) => s.address(),
            Server::Proxy(s) => s.address(),
            Server::Qtv(s) => s.address(),
            Server::Generic(s) => s.address(),
        }
    }
}

pub trait ServerInfo {
    // required
    fn server_type(&self) -> ServerType;
    fn software_type(&self) -> SoftwareType;
    fn ip(&self) -> &str;
    fn port(&self) -> u16;

    // derived
    fn address(&self) -> String {
        format!("{}:{}", self.ip(), self.port())
    }
}

#[cfg(feature = "tokio")]
use super::{svc_qtvusers::qtvusers_async, svc_status::status_119_async};

/// Query a server (sync).
pub fn query(address: &str, timeout: Duration) -> Result<Server> {
    let hostport = HostPort::try_from(address)?;
    let ip = resolve_host(hostport.host())?;
    let status_res = status_119(address, timeout)?;

    let qtv_stream_opt = status_res.qtv_stream().as_ref().map(|stream| {
        let names = qtvusers(address, timeout)
            .unwrap_or_default()
            .client_names();
        stream.with_client_names(&names)
    });

    Ok(build_server(ip, hostport, status_res, qtv_stream_opt))
}

/// Query a server (async).
#[cfg(feature = "tokio")]
pub async fn query_async(address: &str, timeout: Duration) -> Result<Server> {
    let hostport = HostPort::try_from(address)?;
    let ip = resolve_host(hostport.host())?;
    let status_res = status_119_async(address, timeout).await?;

    let qtv_stream_opt = match status_res.qtv_stream() {
        Some(stream) => {
            let names = qtvusers_async(address, timeout)
                .await
                .unwrap_or_default()
                .client_names();
            Some(stream.with_client_names(&names))
        }
        None => None,
    };

    Ok(build_server(ip, hostport, status_res, qtv_stream_opt))
}

fn build_server(
    ip: String,
    hostport: HostPort,
    status_res: Status119Response,
    qtv_stream: Option<QtvStream>,
) -> Server {
    let version = status_res.settings().version.clone().unwrap_or_default();

    let server = GenericServer {
        server_type: ServerType::from_version(&version),
        software_type: SoftwareType::from_version(&version),
        ip,
        port: hostport.port(),
        settings: status_res.settings().clone(),
        clients: status_res.clients().cloned().collect(),
        qtv_stream,
        geo: GeoInfo::from(status_res.settings()),
    };

    match server.server_type() {
        ServerType::QtvServer => Server::Qtv(QtvServer::from(&server)),
        ServerType::GameServer => Server::Game(GameServer::from(&server)),
        ServerType::ProxyServer => Server::Proxy(ProxyServer::from(&server)),
        ServerType::Unknown => Server::Generic(server),
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::common::geo::Coords;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::net::{IpAddr, ToSocketAddrs};

    #[tokio::test]
    async fn test_query() -> Result<()> {
        // invalid address
        assert!(
            query_async("foo.bar:666", Duration::from_millis(50))
                .await
                .is_err()
        );

        // valid address
        {
            let expected_ip = {
                ("berlin2.qwsv.net", 0)
                    .to_socket_addrs()?
                    .find(|addr| matches!(addr.ip(), IpAddr::V4(_)))
                    .map(|addr| addr.ip())
                    .unwrap()
            };
            let timeout = Duration::from_secs_f32(0.5);
            let generic_server = query_async("berlin2.qwsv.net:27500", timeout).await?;

            if let Server::Game(server) = &generic_server {
                assert_eq!(server.server_type(), ServerType::GameServer);
                assert_eq!(server.software_type(), SoftwareType::Mvdsv);
                assert_eq!(server.address(), format!("{expected_ip}:27500"));
                assert_eq!(server.port(), 27500);
                assert_eq!(server.ip(), expected_ip.to_string());

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

            let server2 = query("berlin2.qwsv.net:27500", timeout)?;
            assert_eq!(generic_server, server2);
        }

        Ok(())
    }
}
