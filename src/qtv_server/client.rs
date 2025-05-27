use crate::GenericClient;

/// A client connected to a [`QtvServer`](crate::QtvServer)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QtvClient {
    id: u32,
    time: u32,
    name: String,
}

#[allow(dead_code)]
impl QtvClient {
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

impl From<&GenericClient> for QtvClient {
    fn from(client: &GenericClient) -> Self {
        Self {
            id: client.id(),
            time: client.time(),
            name: client.name().to_string(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_qtvclient_from_quakeclient() {
        let client = QtvClient::from(&GenericClient {
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
