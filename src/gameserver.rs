use crate::client::QuakeClient;
use crate::server::QuakeServer;
use crate::team;
use crate::team::Team;
use quake_serverinfo::Settings;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct GameServer {
    pub settings: Settings,
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
    pub spectators: Vec<Spectator>,
}

impl From<&QuakeServer> for GameServer {
    fn from(server: &QuakeServer) -> Self {
        let mut clients = server.clients.clone();
        clients.sort();

        let players: Vec<Player> = clients
            .iter()
            .filter(|c| !c.is_spectator)
            .map(Player::from)
            .collect();

        let spectators: Vec<Spectator> = clients
            .iter()
            .filter(|c| c.is_spectator)
            .map(Spectator::from)
            .collect();

        let teams = match server.settings.teamplay {
            Some(tp) if tp > 0 => team::from_players(&players),
            _ => vec![],
        };

        Self {
            settings: server.settings.clone(),
            teams,
            players,
            spectators,
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
}

impl From<&QuakeClient> for Spectator {
    fn from(client: &QuakeClient) -> Self {
        Self {
            id: client.id,
            name: client.name.clone(),
            auth_cc: client.auth_cc.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[tokio::test]
    async fn test_from_gameserver() -> Result<()> {
        let server =
            QuakeServer::try_from_address("quake.se:28501", Duration::from_secs_f32(0.5)).await?;
        assert_eq!(
            GameServer::from(&server).settings.hostname,
            Some("QUAKE.SE KTX:28501".to_string())
        );
        Ok(())
    }
}
