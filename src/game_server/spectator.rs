use crate::GenericClient;
use quake_text::unicode;
use std::cmp::Ordering;

/// A client connected to a [`GameServer`](crate::GameServer) as spectator.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Spectator {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) auth_cc: String,
    pub(super) is_bot: bool,
}

impl From<&GenericClient> for Spectator {
    fn from(client: &GenericClient) -> Self {
        Self {
            id: client.id(),
            name: client.name().to_string(),
            is_bot: client.is_bot(),
            auth_cc: client.auth_cc().to_string(),
        }
    }
}

#[allow(dead_code)]
impl Spectator {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn auth_cc(&self) -> &str {
        &self.auth_cc
    }

    pub fn is_bot(&self) -> bool {
        self.is_bot
    }
}

impl PartialOrd for Spectator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Spectator {
    fn cmp(&self, other: &Self) -> Ordering {
        unicode::ord(&self.name, &other.name)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_specator_from_genericclient() {
        let spectator = Spectator::from(&GenericClient {
            id: 7,
            name: "XantoM".to_string(),
            team: "f0m".to_string(),
            frags: 12,
            ping: 25,
            time: 15,
            top_color: 4,
            bottom_color: 2,
            skin: "XantoM".to_string(),
            auth_cc: "xtm".to_string(),
            is_spectator: false,
            is_bot: false,
        });
        assert_eq!(spectator.id(), 7);
        assert_eq!(spectator.name(), "XantoM");
        assert_eq!(spectator.auth_cc(), "xtm");
        assert_eq!(spectator.is_bot(), false);
    }
}
