/// Represents a QTV stream in a [`GameServer`](crate::GameServer)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QtvStream {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) number: Option<u32>,
    pub(crate) address: Option<String>,
    pub(crate) client_count: u32,
    pub(crate) client_names: Vec<String>,
}

#[allow(dead_code)]
impl QtvStream {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn number(&self) -> Option<u32> {
        self.number
    }

    pub fn address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    pub fn client_count(&self) -> u32 {
        self.client_count
    }

    pub fn client_names(&self) -> std::slice::Iter<String> {
        self.client_names.iter()
    }

    pub fn url(&self) -> Option<String> {
        Some(format!("{}@{}", self.number()?, self.address()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_qtvstream() {
        let stream = QtvStream {
            id: 2,
            name: "QUAKE.SE KTX Qtv (1)".to_string(),
            number: Some(1),
            address: Some("quake.se:28000".to_string()),
            client_names: vec![],
            client_count: 0,
        };
        assert_eq!(stream.id(), 2);
        assert_eq!(stream.name(), "QUAKE.SE KTX Qtv (1)");
        assert_eq!(stream.number(), Some(1));
        assert_eq!(stream.address(), Some(&"quake.se:28000".to_string()));
        assert_eq!(stream.client_count(), 0);
        assert_eq!(stream.client_names().count(), 0);
        assert_eq!(stream.url(), Some("1@quake.se:28000".to_string()));
    }
}
