use std::net::{SocketAddr, ToSocketAddrs};

/// Resolves a hostname or IP string to its first IPv4 address.
pub(super) fn host_as_ipv4(host: &str) -> Result<String, Error> {
    (host, 0)
        .to_socket_addrs()
        .map_err(|_| Error::FailedToResolve(host.to_string()))?
        .find_map(|addr| match addr {
            SocketAddr::V4(v4) => Some(v4.ip().to_string()),
            _ => None,
        })
        .ok_or_else(|| Error::FailedToResolve(host.to_string()))
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    InvalidFormat(#[from] hostport::ParseError),

    #[error("failed to resolve '{0}' to an IPv4 address")]
    FailedToResolve(String),
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_host_as_ipv4() -> Result<()> {
        assert_eq!(
            host_as_ipv4("INVALID_ADDRESS").unwrap_err(),
            Error::FailedToResolve("INVALID_ADDRESS".to_string())
        );
        assert_eq!(
            host_as_ipv4("0:0:0:0:0:ffff:c0a8:0001:28000").unwrap_err(),
            Error::FailedToResolve("0:0:0:0:0:ffff:c0a8:0001:28000".to_string())
        );
        assert_eq!(host_as_ipv4("1.2.3.4")?, "1.2.3.4".to_string());
        assert!(
            ["1.1.1.1".to_string(), "1.0.0.1".to_string()]
                .contains(&host_as_ipv4("one.one.one.one")?)
        );
        Ok(())
    }
}
