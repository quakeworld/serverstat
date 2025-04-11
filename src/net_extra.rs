use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};

pub fn address_to_ipv4(address: &str) -> Option<String> {
    let host = address.split_once(':').map_or(address, |(h, _)| h);

    if host.parse::<Ipv4Addr>().is_ok() {
        return Some(host.to_string());
    }

    address
        .to_socket_addrs()
        .ok()?
        .filter_map(|addr| {
            if let SocketAddr::V4(v4_addr) = addr {
                Some(v4_addr.ip().to_string())
            } else {
                None
            }
        })
        .next()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_resolve_ip() -> Result<()> {
        assert_eq!(address_to_ipv4("1.2.3.4"), Some("1.2.3.4".to_string()));
        assert!(
            [Some("1.1.1.1".to_string()), Some("1.0.0.1".to_string())]
                .contains(&address_to_ipv4("one.one.one.one:26000"))
        );
        Ok(())
    }
}
