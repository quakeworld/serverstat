use std::cmp::Ordering;

use quake_text::unicode;

use crate::GenericClient;

/// A client connected to a [`ProxyServer`](crate::ProxyServer)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProxyClient {
    pub(super) id: u32,
    pub(super) time: u32,
    pub(super) name: String,
}

impl From<&GenericClient> for ProxyClient {
    fn from(client: &GenericClient) -> Self {
        Self {
            id: client.id(),
            time: client.time(),
            name: client.name().to_string(),
        }
    }
}

#[allow(dead_code)]
impl ProxyClient {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn time(&self) -> u32 {
        self.time
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PartialOrd for ProxyClient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProxyClient {
    fn cmp(&self, other: &Self) -> Ordering {
        unicode::ord(&self.name, &other.name)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_proxyclient_from_quakeclient() {
        let client = ProxyClient::from(&GenericClient {
            id: 1,
            name: "TestClient".to_string(),
            time: 100,
            ..Default::default()
        });
        assert_eq!(client.id(), 1);
        assert_eq!(client.time(), 100);
        assert_eq!(client.name(), "TestClient");
    }
}
