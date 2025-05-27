use super::team;
use crate::{GenericClient, GenericServer};

use crate::{ClientSlots, GeoInfo, Player, QtvStream, ServerType, SoftwareType, Spectator, Team};
use quake_serverinfo::Settings;
use quake_text::unicode;

/// Represents a server where clients connect as [`Player`] or [`Spectator`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameServer {
    pub(crate) software_type: SoftwareType,
    pub(crate) address: String,
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) settings: Settings,
    pub(crate) teams: Vec<Team>,
    pub(crate) players: Vec<Player>,
    pub(crate) spectators: Vec<Spectator>,
    pub(crate) qtv_stream: Option<QtvStream>,
    pub(crate) geo: GeoInfo,
}

#[allow(dead_code)]
impl GameServer {
    pub fn server_type(&self) -> ServerType {
        ServerType::GameServer
    }

    pub fn software_type(&self) -> SoftwareType {
        self.software_type.clone()
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn ip(&self) -> &str {
        &self.ip
    }
    pub fn port(&self) -> u16 {
        self.port
    }

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

    pub fn geo(&self) -> &GeoInfo {
        &self.geo
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
        let teams = if is_teamplay {
            team::teams_from_players(&players)
        } else {
            vec![]
        };

        Self {
            software_type: server.software_type(),
            address: server.address().to_string(),
            ip: server.ip().to_string(),
            port: server.port(),
            settings: server.settings().to_owned(),
            teams,
            players,
            spectators,
            qtv_stream: server.qtv_stream().cloned(),
            geo: server.geo().to_owned(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::{Coords, GenericClient, GeoInfo, ServerType, SoftwareType};
    use pretty_assertions::assert_eq;
    use quake_serverinfo::Settings;

    #[test]
    fn test_from_genericserver() {
        let generic = GenericServer {
            server_type: ServerType::GameServer,
            software_type: SoftwareType::Mvdsv,
            address: "10.10.10.10".to_string(),
            ip: "10.10.10.10".to_string(),
            port: 28501,
            settings: Settings {
                hostname: Some("LocalMvdsv".to_string()),
                maxclients: Some(4),
                maxspectators: Some(8),
                teamplay: Some(2),
                ..Default::default()
            },
            clients: vec![
                GenericClient {
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    is_spectator: true,
                    ..Default::default()
                },
            ],
            qtv_stream: None,
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                country_name: Some("United States".to_string()),
                city: Some("New York".to_string()),
                region: Some("North America".to_string()),
                coords: Some(Coords::new(40.7128, -74.0060)),
            },
        };
        let server = GameServer::from(&generic);
        assert_eq!(server.server_type(), ServerType::GameServer);
        assert_eq!(server.software_type(), SoftwareType::Mvdsv);
        assert_eq!(server.address(), generic.address());
        assert_eq!(server.ip(), generic.ip());
        assert_eq!(server.port(), generic.port());
        assert_eq!(server.player_slots(), ClientSlots::new(2, 4));
        assert_eq!(server.spectator_slots(), ClientSlots::new(1, 8));
        assert_eq!(server.teams().count(), 1);
        assert_eq!(server.players().count(), 2);
        assert_eq!(server.spectators().count(), 1);
        assert_eq!(server.qtv_stream(), None);
        assert_eq!(server.geo(), generic.geo());

        // no teamplay
        let generic = GenericServer {
            settings: Settings {
                teamplay: Some(0),
                ..Default::default()
            },
            clients: vec![
                GenericClient {
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    is_spectator: false,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let server = GameServer::from(&generic);
        assert_eq!(server.teams().count(), 0);
    }
}
