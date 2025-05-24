use crate::server::quake_client::QuakeClient;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ProxyClient {
    pub(super) id: u32,
    pub(super) time: u32,
    pub(super) name: String,
}

impl From<&QuakeClient> for ProxyClient {
    fn from(client: &QuakeClient) -> Self {
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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_proxyclient_from_quakeclient() {
        let client = ProxyClient::from(&QuakeClient {
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
