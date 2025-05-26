#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a QTV stream in a [`GameServer`](crate::GameServer)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
