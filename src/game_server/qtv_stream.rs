/// Represents a QTV stream in a [`GameServer`](crate::GameServer)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct QtvStream {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) number: Option<u32>,
    pub(crate) address: Option<String>,
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

    pub fn url(&self) -> Option<String> {
        Some(format!("{}@{}", self.number()?, self.address()?))
    }

    pub fn client_names(&self) -> &[String] {
        &self.client_names
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QtvStream {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("QtvStream", 6)?;
        state.serialize_field("id", &self.id())?;
        state.serialize_field("name", &self.name())?;
        state.serialize_field("number", &self.number())?;
        state.serialize_field("address", &self.address())?;
        state.serialize_field("url", &self.url())?;
        state.serialize_field("client_names", self.client_names())?;
        state.end()
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
        };
        assert_eq!(stream.id(), 2);
        assert_eq!(stream.name(), "QUAKE.SE KTX Qtv (1)");
        assert_eq!(stream.number(), Some(1));
        assert_eq!(stream.address(), Some(&"quake.se:28000".to_string()));
        assert_eq!(stream.url(), Some("1@quake.se:28000".to_string()));
        assert_eq!(stream.client_names().len(), 0);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() -> anyhow::Result<()> {
        let qtv = QtvStream {
            id: 2,
            name: "QUAKE.SE KTX Qtv (1)".to_string(),
            number: Some(1),
            address: Some("quake.se:28000".to_string()),
            client_names: vec!["XantoM".to_string()],
        };

        let qtv_json = r#"{"id":2,"name":"QUAKE.SE KTX Qtv (1)","number":1,"address":"quake.se:28000","url":"1@quake.se:28000","client_names":["XantoM"]}"#;

        // ensure round-trip serialization
        assert_eq!(serde_json::to_string(&qtv)?, qtv_json);
        assert_eq!(serde_json::from_str::<QtvStream>(qtv_json)?, qtv);

        Ok(())
    }
}
