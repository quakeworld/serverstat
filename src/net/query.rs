use super::resolve::{self, host_as_ipv4};
use super::svc_qtvusers::{self, QtvusersResponse};
use super::svc_status::{self, StatusResponse};
use crate::{
    GameServer, GenericServer, GeoInfo, ProxyServer, QtvServer, QtvStream, Server, ServerType,
    SoftwareType,
};
use hostport::HostPort;
use std::time::Duration;

/// Query a server (sync).
pub fn serverinfo(address: &str, timeout: Duration) -> Result<Server, QueryError> {
    let hostport = HostPort::try_from(address)?;
    let ip = host_as_ipv4(hostport.host())?;
    let status_response = query_status(address, timeout)?;

    let qtv_client_names = if status_response
        .qtv_stream()
        .as_ref()
        .is_some_and(|stream| stream.client_count > 0)
    {
        query_qtvusers(address, timeout)
            .map(|res| res.client_names().to_vec())
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    Ok(compose_server(
        ip,
        hostport,
        status_response,
        qtv_client_names,
    ))
}

/// Query a server (async).
#[cfg(feature = "tokio")]
pub async fn serverinfo_async(address: &str, timeout: Duration) -> Result<Server, QueryError> {
    let hostport = HostPort::try_from(address)?;
    let ip = host_as_ipv4(hostport.host())?;
    let status_response = query_status_async(address, timeout).await?;

    let qtv_client_names = if status_response
        .qtv_stream()
        .as_ref()
        .is_some_and(|stream| stream.client_count > 0)
    {
        query_qtvusers_async(address, timeout)
            .await
            .map(|res| res.client_names().to_vec())
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    Ok(compose_server(
        ip,
        hostport,
        status_response,
        qtv_client_names,
    ))
}

fn compose_server(
    ip: String,
    hostport: HostPort,
    status_res: StatusResponse,
    qtv_client_names: Vec<String>,
) -> Server {
    let version = status_res.settings().version.clone().unwrap_or_default();
    let qtv_stream = status_res.qtv_stream().as_ref().map(|s| QtvStream {
        id: s.id,
        name: s.name.to_string(),
        number: s.number,
        client_count: s.client_count,
        address: s.address.clone(),
        client_names: qtv_client_names,
    });

    let server = GenericServer {
        server_type: ServerType::from_version(&version),
        software_type: SoftwareType::from_version(&version),
        ip,
        port: hostport.port(),
        settings: status_res.settings().clone(),
        clients: status_res.clients().into(),
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

// query for status (svc_status)
fn query_status(address: &str, timeout: Duration) -> Result<StatusResponse, QueryError> {
    let options = tinyudp::ReadOptions::new(timeout, svc_status::BUFFER_SIZE);
    let response = tinyudp::send_and_receive(address, svc_status::COMMAND, options)?;
    Ok(StatusResponse::try_from(response.as_slice())?)
}

#[cfg(feature = "tokio")]
async fn query_status_async(
    address: &str,
    timeout: Duration,
) -> Result<StatusResponse, QueryError> {
    let options = tinyudp::ReadOptions::new(timeout, svc_status::BUFFER_SIZE);
    let response = tinyudp::send_and_receive_async(address, svc_status::COMMAND, options).await?;
    Ok(StatusResponse::try_from(response.as_slice())?)
}

/// query for qtvusers (svc_qtvusers)
fn query_qtvusers(address: &str, timeout: Duration) -> Result<QtvusersResponse, QueryError> {
    let options = tinyudp::ReadOptions::new(timeout, svc_qtvusers::BUFFER_SIZE);
    let response = tinyudp::send_and_receive(address, svc_qtvusers::COMMAND, options)?;
    Ok(QtvusersResponse::try_from(response.as_slice())?)
}

#[cfg(feature = "tokio")]
async fn query_qtvusers_async(
    address: &str,
    timeout: Duration,
) -> Result<QtvusersResponse, QueryError> {
    let options = tinyudp::ReadOptions::new(timeout, svc_qtvusers::BUFFER_SIZE);
    let response = tinyudp::send_and_receive_async(address, svc_qtvusers::COMMAND, options).await?;
    Ok(QtvusersResponse::try_from(response.as_slice())?)
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error(transparent)]
    InvalidAddress(#[from] hostport::ParseError),

    #[error(transparent)]
    ResolveAddress(#[from] resolve::Error),

    #[error(transparent)]
    Udp(#[from] tinyudp::Error),

    #[error(transparent)]
    StatusQuery(#[from] svc_status::Error),

    #[error(transparent)]
    QtvusersQuery(#[from] svc_qtvusers::Error),
}

#[cfg(test)]
#[cfg(feature = "tokio")]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::net::{IpAddr, ToSocketAddrs};

    #[tokio::test]
    async fn test_serverinfo() -> Result<()> {
        // invalid address
        assert!(
            serverinfo_async("foo.bar:666", Duration::from_millis(50))
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
            let timeout = Duration::from_secs(1);
            let generic_server = serverinfo_async("de.quake.world:28501", timeout).await?;

            if let Server::Game(server) = &generic_server {
                assert_eq!(server.server_type(), ServerType::GameServer);
                assert_eq!(server.software_type(), SoftwareType::Mvdsv);
                assert_eq!(server.address(), format!("{expected_ip}:28501"));
                assert_eq!(server.port(), 28501);
                assert_eq!(server.ip(), expected_ip);

                let settings = server.settings().clone();
                assert!(settings.hostname.unwrap().contains("de.quake.world:28501"));
            }

            assert_eq!(generic_server, serverinfo("de.quake.world:28501", timeout)?);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_query_status() -> Result<()> {
        assert_eq!(
            query_status_async("quake.se:28501", Duration::from_secs(1)).await?,
            query_status("quake.se:28501", Duration::from_secs(1))?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_query_qtvusers() -> Result<()> {
        assert_eq!(
            query_qtvusers_async("quake.se:28501", Duration::from_secs(1)).await?,
            query_qtvusers("quake.se:28501", Duration::from_secs(1))?
        );
        Ok(())
    }
}
