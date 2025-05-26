use super::resolve::{self, host_as_ipv4};
use super::svc_status::{self, Status119Response, query_status_119};
use crate::{
    GameServer, GenericServer, GeoInfo, ProxyServer, QtvServer, Server, ServerType, SoftwareType,
};
use hostport::HostPort;
use std::time::Duration;

#[cfg(feature = "tokio")]
use super::svc_status::query_status_119_async;

/// Query a server (sync).
pub fn query_server(address: &str, timeout: Duration) -> Result<Server, Error> {
    let hostport = HostPort::try_from(address)?;
    let ip = host_as_ipv4(hostport.host())?;
    let status_res = query_status_119(address, timeout)?;
    Ok(compose_server(ip, hostport, status_res))
}

/// Query a server (async).
#[cfg(feature = "tokio")]
pub async fn query_server_async(address: &str, timeout: Duration) -> Result<Server, Error> {
    let hostport = HostPort::try_from(address)?;
    let ip = host_as_ipv4(hostport.host())?;
    let status_res = query_status_119_async(address, timeout).await?;
    Ok(compose_server(ip, hostport, status_res))
}

fn compose_server(ip: String, hostport: HostPort, status_res: Status119Response) -> Server {
    let version = status_res.settings().version.clone().unwrap_or_default();
    let server = GenericServer {
        server_type: ServerType::from_version(&version),
        software_type: SoftwareType::from_version(&version),
        address: format!("{}:{}", ip, hostport.port()),
        ip,
        port: hostport.port(),
        settings: status_res.settings().clone(),
        clients: status_res.clients().cloned().collect(),
        qtv_stream: None,
        geo: GeoInfo::from(status_res.settings()),
    };

    match server.server_type() {
        ServerType::QtvServer => Server::Qtv(QtvServer::from(&server)),
        ServerType::GameServer => Server::Game(GameServer::from(&server)),
        ServerType::ProxyServer => Server::Proxy(ProxyServer::from(&server)),
        ServerType::Unknown => Server::Generic(server),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    AddressParse(#[from] hostport::ParseError),

    #[error(transparent)]
    AddressResolve(#[from] resolve::Error),

    #[error(transparent)]
    StatusQuery(#[from] svc_status::Status119ResponseError),
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
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
                ("de.quake.world", 0)
                    .to_socket_addrs()?
                    .find(|addr| matches!(addr.ip(), IpAddr::V4(_)))
                    .map(|addr| addr.ip().to_string())
                    .unwrap()
            };
            let timeout = Duration::from_secs_f32(0.5);
            let generic_server = query_server_async("de.quake.world:28501", timeout).await?;

            if let Server::Game(server) = &generic_server {
                assert_eq!(server.server_type(), ServerType::GameServer);
                assert_eq!(server.software_type(), SoftwareType::Mvdsv);
                assert_eq!(server.address(), format!("{expected_ip}:28501"));
                assert_eq!(server.port(), 28501);
                assert_eq!(server.ip(), expected_ip);

                let settings = server.settings().clone();
                assert!(settings.hostname.unwrap().contains("de.quake.world:28501"));
            }

            let server_sync = query_server("de.quake.world:28501", timeout)?;
            assert_eq!(generic_server, server_sync);
        }

        Ok(())
    }
}
