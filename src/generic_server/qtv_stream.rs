use super::tokenize::tokenize;
use quake_text::bytestr::to_unicode;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a QTV stream in a [`GameServer`](crate::GameServer)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        match (self.number(), self.address()) {
            (Some(number), Some(address)) => Some(format!("{}@{}", number, address)),
            _ => None,
        }
    }
}

impl TryFrom<&[u8]> for QtvStream {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let parts: Vec<String> = tokenize(to_unicode(bytes).as_str());
        let id = parts[1].parse::<u32>().unwrap_or_default();
        let name = parts[2].to_string();
        let url = parts[3].to_string();
        let (number, address) = url
            .split_once('@')
            .map(|(num_str, addr)| (num_str.parse::<u32>().ok(), Some(addr.to_string())))
            .unwrap_or((None, None));
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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_try_from_bytes() -> Result<()> {
        // empty url
        {
            let bytes = br#"qtv 2 "QUAKE.SE KTX Qtv (1)" "" 0"#.as_slice();
            let stream = QtvStream::try_from(bytes)?;
            assert_eq!(stream.id(), 2);
            assert_eq!(stream.name(), "QUAKE.SE KTX Qtv (1)");
            assert_eq!(stream.number(), None);
            assert_eq!(stream.address(), None);
            assert_eq!(stream.client_count(), 0);
            assert!(stream.client_names.is_empty());
            assert_eq!(stream.url(), None);
        }

        // valid
        {
            let bytes = br#"qtv 2 "QUAKE.SE KTX Qtv (1)" "1@quake.se:28000" 0"#.as_slice();
            let stream = QtvStream::try_from(bytes)?;
            assert_eq!(stream.id(), 2);
            assert_eq!(stream.name(), "QUAKE.SE KTX Qtv (1)");
            assert_eq!(stream.number(), Some(1));
            assert_eq!(stream.address(), Some(&"quake.se:28000".to_string()));
            assert_eq!(stream.client_count(), 0);
            assert!(stream.client_names.is_empty());
            assert_eq!(stream.url(), Some("1@quake.se:28000".to_string()));
        }

        Ok(())
    }
}
