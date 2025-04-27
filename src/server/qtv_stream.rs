use crate::util::tokenize;
use hostport::HostPort;
use quake_text::bytestr::to_unicode;

#[cfg(feature = "json")]
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Deserialize))]
pub struct QtvStream {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) number: Option<u32>,
    pub(crate) address: Option<HostPort>,
    pub(crate) client_count: u32,
    pub(crate) client_names: Vec<String>,
}

#[allow(dead_code)]
impl QtvStream {
    pub fn with_client_names(&self, client_names: &[String]) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            number: self.number,
            address: self.address.clone(),
            client_count: self.client_count,
            client_names: client_names.to_vec(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn number(&self) -> Option<u32> {
        self.number
    }

    pub fn address(&self) -> Option<&HostPort> {
        self.address.as_ref()
    }

    pub fn client_count(&self) -> u32 {
        self.client_count
    }

    pub fn client_names(&self) -> &[String] {
        &self.client_names
    }

    pub fn url(&self) -> Option<String> {
        match (self.number(), self.address()) {
            (Some(number), Some(address)) => Some(format!("{}@{}", number, address)),
            _ => None,
        }
    }
}

impl TryFrom<&[u8]> for QtvStream {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let parts: Vec<String> = tokenize::tokenize(to_unicode(bytes).as_str());
        let id = parts[1].parse::<u32>().unwrap_or_default();
        let name = parts[2].to_string();
        let url = parts[3].to_string();
        let (number, address) = match url.split_once('@') {
            Some((number_str, hostport)) => {
                let number = number_str.parse::<u32>().unwrap_or_default();
                (Some(number), HostPort::try_from(hostport).ok())
            }
            None => (None, None),
        };
        let client_count = parts[4].parse::<u32>().unwrap_or_default();

        Ok(Self {
            id,
            name,
            number,
            address,
            client_count,
            client_names: vec![],
        })
    }
}

#[cfg(feature = "json")]
#[cfg_attr(coverage_nightly, coverage(off))]
impl Serialize for QtvStream {
    fn serialize<S>(&self, serializer: S) -> anyhow::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("QtvStream", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("number", &self.number)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("url", &self.url())?;
        state.serialize_field("client_count", &self.client_count)?;
        state.serialize_field("client_names", &self.client_names)?;
        state.end()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use anyhow::Result;
    use hostport::HostPort;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_try_from_bytes() -> Result<()> {
        // empty url
        {
            let bytes = br#"qtv 2 "QUAKE.SE KTX Qtv (1)" "" 0"#.as_slice();
            let stream = QtvStream::try_from(bytes)?;
            assert_eq!(stream.id, 2);
            assert_eq!(stream.name, "QUAKE.SE KTX Qtv (1)");
            assert_eq!(stream.number, None);
            assert_eq!(stream.address, None);
            assert_eq!(stream.client_count, 0);
            assert!(stream.client_names.is_empty());
            assert_eq!(stream.url(), None);
        }

        // valid
        {
            let bytes = br#"qtv 2 "QUAKE.SE KTX Qtv (1)" "1@quake.se:28000" 0"#.as_slice();
            let stream = QtvStream::try_from(bytes)?;
            assert_eq!(stream.id, 2);
            assert_eq!(stream.name, "QUAKE.SE KTX Qtv (1)");
            assert_eq!(stream.number, Some(1));
            assert_eq!(stream.address, Some(HostPort::new("quake.se", 28000)?));
            assert_eq!(stream.client_count, 0);
            assert!(stream.client_names.is_empty());
            assert_eq!(stream.url(), Some("1@quake.se:28000".to_string()));
        }

        Ok(())
    }

    #[test]
    fn test_serialize() -> Result<()> {
        let stream = QtvStream {
            id: 2,
            name: "QUAKE.SE KTX Qtv (1)".to_string(),
            number: Some(1),
            address: Some(HostPort::new("quake.se", 28000)?),
            client_count: 0,
            client_names: vec![],
        };

        assert_eq!(
            serde_json::to_string(&stream)?,
            r#"{"id":2,"name":"QUAKE.SE KTX Qtv (1)","number":1,"address":"quake.se:28000","url":"1@quake.se:28000","client_count":0,"client_names":[]}"#
        );
        Ok(())
    }
}
