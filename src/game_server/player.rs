use crate::generic_server::client::QuakeClient;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Player {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) team: String,
    pub(super) frags: i32,
    pub(super) ping: u32,
    pub(super) time: u32,
    pub(super) top_color: u8,
    pub(super) bottom_color: u8,
    pub(super) skin: String,
    pub(super) auth_cc: String,
    pub(super) is_bot: bool,
}

impl From<&QuakeClient> for Player {
    fn from(client: &QuakeClient) -> Self {
        Self {
            id: client.id(),
            name: client.name().to_string(),
            team: client.team().to_string(),
            frags: client.frags(),
            ping: client.ping(),
            time: client.time(),
            top_color: client.top_color(),
            bottom_color: client.bottom_color(),
            skin: client.skin().to_string(),
            is_bot: client.is_bot(),
            auth_cc: client.auth_cc().to_string(),
        }
    }
}

#[allow(dead_code)]
impl Player {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn team(&self) -> &str {
        &self.team
    }

    pub fn frags(&self) -> i32 {
        self.frags
    }

    pub fn ping(&self) -> u32 {
        self.ping
    }

    pub fn time(&self) -> u32 {
        self.time
    }

    pub fn top_color(&self) -> u8 {
        self.top_color
    }

    pub fn bottom_color(&self) -> u8 {
        self.bottom_color
    }

    pub fn skin(&self) -> &str {
        &self.skin
    }

    pub fn auth_cc(&self) -> &str {
        &self.auth_cc
    }

    pub fn is_bot(&self) -> bool {
        self.is_bot
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_player_from_quakeclient() {
        let player = Player::from(&QuakeClient {
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
        assert_eq!(player.id(), 7);
        assert_eq!(player.name(), "XantoM");
        assert_eq!(player.team(), "f0m");
        assert_eq!(player.frags(), 12);
        assert_eq!(player.ping(), 25);
        assert_eq!(player.time(), 15);
        assert_eq!(player.top_color(), 4);
        assert_eq!(player.bottom_color(), 2);
        assert_eq!(player.skin(), "XantoM");
        assert_eq!(player.auth_cc(), "xtm");
        assert_eq!(player.is_bot(), false);
    }
}
