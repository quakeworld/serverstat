//! Game server (clients can connect as player or spectator)
use super::team;
use crate::generic_server::client::GenericClient;
use crate::generic_server::server::GenericServer;
use crate::{ClientSlots, GeoInfo, Player, QtvStream, Spectator, Team};
use quake_serverinfo::Settings;
use quake_text::unicode;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a game server (a client can connect as [`Player`] or [`Spectator`]).
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameServer {
    settings: Settings,
    teams: Vec<Team>,
    players: Vec<Player>,
    spectators: Vec<Spectator>,
    qtv_stream: Option<QtvStream>,
    geo: GeoInfo,
}

#[allow(dead_code)]
impl GameServer {
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn teams(&self) -> impl Iterator<Item = &Team> {
        self.teams.iter()
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.players.iter()
    }

    pub fn spectators(&self) -> impl Iterator<Item = &Spectator> {
        self.spectators.iter()
    }

    pub fn qtv_stream(&self) -> Option<&QtvStream> {
        self.qtv_stream.as_ref()
    }

    pub fn geo(&self) -> &GeoInfo {
        &self.geo
    }

    pub fn player_slots(&self) -> ClientSlots {
        let used = self.players.len() as u32;
        let total = self.settings.maxclients.map(|v| v as u32).unwrap_or(used);
        ClientSlots::new(used, total)
    }

    pub fn spectator_slots(&self) -> ClientSlots {
        let used = self.spectators.len() as u32;
        let total = self
            .settings
            .maxspectators
            .map(|v| v as u32)
            .unwrap_or(used);
        ClientSlots::new(used, total)
    }
}

impl From<&GenericServer> for GameServer {
    fn from(server: &GenericServer) -> Self {
        let mut clients: Vec<GenericClient> = server.clients().cloned().collect();
        clients.sort();

        let is_teamplay = server.settings().teamplay.is_some_and(|tp| tp > 0);
        let mut players: Vec<Player> = server.players().map(Player::from).collect();

        if is_teamplay {
            players.sort_by(|a, b| unicode::ord(&a.team, &b.team));
        }

        let spectators: Vec<Spectator> = server.spectators().map(Spectator::from).collect();
        let teams = match is_teamplay {
            true => team::teams_from_players(&players),
            _ => vec![],
        };

        Self {
            settings: server.settings().clone(),
            teams,
            players,
            spectators,
            qtv_stream: server.qtv_stream().cloned(),
            geo: server.geo().clone(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::common::geo::Coords;
    use crate::common::server_type::ServerType;
    use crate::common::software_type::SoftwareType;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_from_gameserver() -> Result<()> {
        let geo = GeoInfo {
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
            city: Some("New York".to_string()),
            region: Some("North America".to_string()),
            coords: Some(Coords::new(40.7128, -74.0060)),
        };
        let server = GameServer::from(&GenericServer {
            server_type: ServerType::GameServer,
            software_type: SoftwareType::Mvdsv,
            ip: "10.10.10.10".to_string(),
            port: 28501,
            settings: Settings {
                hostname: Some("LocalQuake".to_string()),
                maxclients: Some(8),
                maxspectators: Some(6),
                teamplay: Some(2),
                ..Default::default()
            },
            clients: vec![
                GenericClient {
                    name: "Player1".to_string(),
                    team: "red".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    name: "Player2".to_string(),
                    team: "blue".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    name: "Spectator1".to_string(),
                    is_spectator: true,
                    ..Default::default()
                },
            ],
            qtv_stream: None,
            geo: geo.clone(),
        });
        assert_eq!(server.settings().hostname, Some("LocalQuake".to_string()));
        assert_eq!(server.settings().maxclients, Some(8));
        assert_eq!(server.settings().maxspectators, Some(6));
        assert_eq!(server.teams.len(), 2);
        assert_eq!(server.players().count(), 2);
        assert_eq!(server.spectators().count(), 1);
        assert_eq!(server.qtv_stream(), None);
        assert_eq!(server.player_slots(), ClientSlots::new(2, 8));
        assert_eq!(server.spectator_slots(), ClientSlots::new(1, 6));
        assert_eq!(server.spectator_slots(), ClientSlots::new(1, 6));
        assert_eq!(server.geo(), &geo);
        Ok(())
    }
}
