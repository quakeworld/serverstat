use crate::client::QuakeClient;
use crate::qtv::QtvStream;
use crate::server::QuakeServer;
use crate::team;
use crate::team::Team;
pub use quake_serverinfo::Settings;
use quake_text::unicode;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct GameServer {
    pub settings: Settings,
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
    pub spectators: Vec<Spectator>,
    pub qtv_stream: Option<QtvStream>,
}

impl From<&QuakeServer> for GameServer {
    fn from(server: &QuakeServer) -> Self {
        let mut clients = server.clients.clone();
        clients.sort();

        let is_teamplay = server.settings.teamplay.is_some_and(|tp| tp > 0);

        let mut players: Vec<Player> = clients
            .iter()
            .filter(|c| !c.is_spectator)
            .map(Player::from)
            .collect();

        if is_teamplay {
            players.sort_by(|a, b| unicode::ord(&a.team, &b.team));
        }

        let spectators: Vec<Spectator> = clients
            .iter()
            .filter(|c| c.is_spectator)
            .map(Spectator::from)
            .collect();

        let teams = match is_teamplay {
            true => team::from_players(&players),
            _ => vec![],
        };

        Self {
            settings: server.settings.clone(),
            teams,
            players,
            spectators,
            qtv_stream: server.qtv_stream.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub team: String,
    pub frags: i32,
    pub ping: u32,
    pub time: u32,
    pub top_color: u8,
    pub bottom_color: u8,
    pub skin: String,
    pub auth_cc: String,
    pub is_bot: bool,
}

impl From<&QuakeClient> for Player {
    fn from(client: &QuakeClient) -> Self {
        Self {
            id: client.id,
            name: client.name.clone(),
            team: client.team.clone(),
            frags: client.frags,
            ping: client.ping,
            time: client.time,
            top_color: client.top_color,
            bottom_color: client.bottom_color,
            skin: client.skin.clone(),
            is_bot: client.is_bot,
            auth_cc: client.auth_cc.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Spectator {
    pub id: u32,
    pub name: String,
    pub auth_cc: String,
    pub is_bot: bool,
}

impl From<&QuakeClient> for Spectator {
    fn from(client: &QuakeClient) -> Self {
        Self {
            id: client.id,
            name: client.name.clone(),
            is_bot: client.is_bot,
            auth_cc: client.auth_cc.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::time::Duration;

    #[tokio::test]
    async fn test_from_gameserver() -> Result<()> {
        let server =
            QuakeServer::try_from_address("quake.se:28501", Duration::from_secs_f32(0.5)).await?;
        assert!(
            GameServer::from(&server)
                .settings
                .hostname
                .unwrap()
                .starts_with("QUAKE.SE KTX:28501"),
        );
        Ok(())
    }

    #[test]
    fn test_player_from_quakeclient() {
        assert_eq!(
            Player::from(&QuakeClient {
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
            }),
            Player {
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
                is_bot: false,
            }
        );
    }

    #[test]
    fn test_specator_from_quakeclient() {
        assert_eq!(
            Spectator::from(&QuakeClient {
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
            }),
            Spectator {
                id: 7,
                name: "XantoM".to_string(),
                auth_cc: "xtm".to_string(),
                is_bot: false,
            }
        );
    }
}
