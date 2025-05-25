use super::resolve::resolve_host;
use super::svc_qtvusers::qtvusers;
use super::svc_status::{Status119Response, status_119};
use crate::{
    GameServer, GenericServer, GeoInfo, ProxyServer, QtvServer, QtvStream, Server, ServerType,
    SoftwareType,
};
use anyhow::Result;
use hostport::HostPort;
use std::time::Duration;

#[cfg(feature = "tokio")]
use super::{svc_qtvusers::qtvusers_async, svc_status::status_119_async};

/// Query a server (sync).
pub fn query_server(address: &str, timeout: Duration) -> Result<Server> {
    let hostport = HostPort::try_from(address)?;
    let ip = resolve_host(hostport.host())?;
    let status_res = status_119(address, timeout)?;

    let qtv_stream_opt = status_res.qtv_stream().map(|stream| {
        stream.client_names = qtvusers(address, timeout)
            .unwrap_or_default()
            .client_names();
        stream
    });

    Ok(build_server(ip, hostport, status_res, &qtv_stream_opt))
}

/// Query a server (async).
#[cfg(feature = "tokio")]
pub async fn query_server_async(address: &str, timeout: Duration) -> Result<Server> {
    let hostport = HostPort::try_from(address)?;
    let ip = resolve_host(hostport.host())?;
    let status_res = status_119_async(address, timeout).await?;

    let qtv_stream_opt = match status_res.qtv_stream() {
        Some(mut stream) => {
            stream.client_names = qtvusers_async(address, timeout)
                .await
                .unwrap_or_default()
                .client_names();
            Some(stream)
        }
        None => None,
    };

    Ok(build_server(ip, hostport, status_res, &qtv_stream_opt))
}

fn build_server(
    ip: String,
    hostport: HostPort,
    status_res: Status119Response,
    qtv_stream: &Option<QtvStream>,
) -> Server {
    let version = status_res.settings().version.clone().unwrap_or_default();

    let server = GenericServer {
        server_type: ServerType::from_version(&version),
        software_type: SoftwareType::from_version(&version),
        address: format!("{}:{}", ip, hostport.port()),
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
            query_server_async("foo.bar:666", Duration::from_millis(50))
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
            let generic_server = query_server_async("berlin2.qwsv.net:27500", timeout).await?;

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

            let server2 = query_server("berlin2.qwsv.net:27500", timeout)?;
            assert_eq!(generic_server, server2);
        }

        Ok(())
    }
}
