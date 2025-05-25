use anyhow::{Result, anyhow as e};
use std::net::{SocketAddr, ToSocketAddrs};

/// Resolves a hostname or IP string to an IPv4 address.
pub(super) fn resolve_host(host: &str) -> Result<String> {
    (host, 0)
        .to_socket_addrs()
        .map_err(|_| e!("Failed to resolve '{}' to an IPv4 address.", host))?
        .find_map(|addr| match addr {
            SocketAddr::V4(v4) => Some(v4.ip().to_string()),
            _ => None,
        })
        .ok_or_else(|| e!("Failed to resolve '{}' to an IPv4 address.", host))
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_resolve_host() -> Result<()> {
        assert!(resolve_host("INVALID_ADDRESS").is_err());
        assert!(resolve_host("0:0:0:0:0:ffff:c0a8:0001:28000").is_err());
        assert_eq!(resolve_host("1.2.3.4")?, "1.2.3.4".to_string());
        assert!(
            ["1.1.1.1".to_string(), "1.0.0.1".to_string()]
                .contains(&resolve_host("one.one.one.one")?)
        );
        Ok(())
    }
}
