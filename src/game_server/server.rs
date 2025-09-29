use super::team;
use crate::{
    GameServerSettings, GenericServer, GeoInfo, Player, QtvStream, ServerType, SoftwareType,
    Spectator, Team,
};
use quake_text::unicode;
use std::cmp::Reverse;

#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

/// Represents a server where clients connect as [`Player`] or [`Spectator`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct GameServer {
    pub(crate) software_type: SoftwareType,
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) settings: GameServerSettings,
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

    pub fn address(&self) -> String {
        format!("{}:{}", self.ip(), self.port())
    }

    pub fn ip(&self) -> &str {
        &self.ip
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn mode(&self) -> String {
        self.settings().mode().unwrap_or("ffa").to_string()
    }

    pub fn settings(&self) -> &GameServerSettings {
        &self.settings
    }

    pub fn teams(&self) -> &[Team] {
        &self.teams
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn spectators(&self) -> &[Spectator] {
        &self.spectators
    }

    pub fn total_spectator_count(&self) -> u32 {
        let qtv_spec_count = self.qtv_stream().map_or(0, |q| q.client_names().len());
        (self.spectators().len() + qtv_spec_count) as u32
    }

    pub fn qtv_stream(&self) -> Option<&QtvStream> {
        self.qtv_stream.as_ref()
    }

    pub fn geo(&self) -> &GeoInfo {
        &self.geo
    }

    #[cfg(feature = "score")]
    pub fn score(&self) -> u32 {
        super::score::from_game_server(self)
    }
}

impl From<&GenericServer> for GameServer {
    fn from(server: &GenericServer) -> Self {
        // settings
        let settings = GameServerSettings::from(&server.settings);

        // players
        let mut players: Vec<Player> = server.players().map(Player::from).collect();

        // teams
        let mut teams = if settings.teamplay() > 0 {
            team::teams_from_players(&players)
        } else {
            vec![]
        };

        // player and team sorting
        let is_standby = settings.status.clone().is_some_and(|s| s == "Standby");
        teams.sort();
        players.sort();

        if !is_standby {
            teams.sort_by_key(|t| Reverse(t.frags()));
            players.sort_by_key(|a| Reverse(a.frags()));
        } else if settings.teamplay() > 0 {
            players.sort_by(|a, b| unicode::ord(a.team(), b.team()));
        }

        // spectators
        let mut spectators: Vec<Spectator> = server.spectators().map(Spectator::from).collect();
        spectators.sort();

        Self {
            software_type: server.software_type(),
            ip: server.ip().to_string(),
            port: server.port(),
            settings,
            teams,
            players,
            spectators,
            qtv_stream: server.qtv_stream().cloned(),
            geo: server.geo().to_owned(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for GameServer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("GameServer", 16)?;
        state.serialize_field("server_type", &self.server_type())?;
        state.serialize_field("software_type", &self.software_type())?;
        state.serialize_field("address", &self.address())?;
        state.serialize_field("ip", self.ip())?;
        state.serialize_field("port", &self.port())?;
        state.serialize_field("mode", &self.mode())?;
        state.serialize_field("settings", self.settings())?;
        state.serialize_field("client_count", &self.players().len())?;
        state.serialize_field("client_limit", &self.settings().maxclients)?;
        state.serialize_field("teams", self.teams())?;
        state.serialize_field("players", self.players())?;
        state.serialize_field("spectators", self.spectators())?;
        state.serialize_field("total_spectator_count", &self.total_spectator_count())?;
        state.serialize_field("qtv_stream", &self.qtv_stream())?;
        state.serialize_field("geo", self.geo())?;

        #[cfg(feature = "score")]
        state.serialize_field("score", &self.score())?;

        state.end()
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
            software_type: SoftwareType::Mvdsv,
            ip: "10.10.10.10".to_string(),
            port: 28501,
            settings: Settings {
                hostname: Some("LocalMvdsv".to_string()),
                maxclients: Some(4),
                maxspectators: Some(8),
                status: Some("Standby".to_string()),
                teamplay: Some(2),
                mode: Some("2on2".to_string()),
                ..Default::default()
            },
            clients: vec![
                GenericClient {
                    name: "abc".to_string(),
                    team: "red".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    name: "vikpe".to_string(),
                    team: "blue".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    name: "XantoM".to_string(),
                    team: "blue".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    name: "qhlan-cam".to_string(),
                    is_spectator: true,
                    ..Default::default()
                },
                GenericClient {
                    name: "[streambot]".to_string(),
                    is_spectator: true,
                    ..Default::default()
                },
                GenericClient {
                    name: "[ServeMe]".to_string(),
                    is_spectator: true,
                    ..Default::default()
                },
            ],
            qtv_stream: Some(QtvStream {
                id: 1,
                name: "Local QTV".to_string(),
                number: Some(1),
                address: Some("10.10.10.10:28000".to_string()),
                client_names: vec!["hub".to_string()],
            }),
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
        assert_eq!(server.mode(), "2on2".to_string());
        assert_eq!(server.teams().len(), 2);
        assert_eq!(server.teams()[0].name(), "blue".to_string()); // ordered by name
        assert_eq!(server.players().len(), 3);
        assert_eq!(server.players()[0].name(), "vikpe".to_string()); // ordered by team name, then player name
        assert_eq!(server.spectators().len(), 3);
        assert_eq!(server.spectators()[0].name(), "[ServeMe]".to_string()); // ordered by name
        assert_eq!(server.total_spectator_count(), 4);
        assert_eq!(server.qtv_stream(), generic.qtv_stream());
        assert_eq!(server.geo(), generic.geo());

        // standby (no teamplay) - players ordered by name
        let generic = GenericServer {
            settings: Settings {
                teamplay: Some(0),
                status: Some("Standby".to_string()),
                ..Default::default()
            },
            clients: vec![
                GenericClient {
                    name: "abc".to_string(),
                    team: "red".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    name: "vikpe".to_string(),
                    team: "blue".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    name: "XantoM".to_string(),
                    team: "blue".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let server = GameServer::from(&generic);
        assert_eq!(server.teams().len(), 0);
        assert_eq!(server.players()[0].name(), "abc".to_string());

        // standby (teamplay) - players ordered by team then name
        let generic = GenericServer {
            settings: Settings {
                teamplay: Some(2),
                status: Some("Standby".to_string()),
                ..Default::default()
            },
            clients: vec![
                GenericClient {
                    name: "abc".to_string(),
                    team: "red".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    name: "vikpe".to_string(),
                    team: "blue".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
                GenericClient {
                    name: "XantoM".to_string(),
                    team: "blue".to_string(),
                    is_spectator: false,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let server = GameServer::from(&generic);
        assert_eq!(server.players()[0].name(), "vikpe".to_string());
        assert_eq!(server.players()[1].name(), "XantoM".to_string());
        assert_eq!(server.players()[2].name(), "abc".to_string());

        // game in progress - players and teams ordered by frags
        let generic = GenericServer {
            settings: Settings {
                teamplay: Some(2),
                status: Some("1 minute left".to_string()),
                ..Default::default()
            },
            clients: vec![
                GenericClient {
                    name: "abc".to_string(),
                    team: "blue".to_string(),
                    frags: 0,
                    ..Default::default()
                },
                GenericClient {
                    name: "XantoM".to_string(),
                    team: "red".to_string(),
                    frags: 8,
                    ..Default::default()
                },
                GenericClient {
                    name: "vikpe".to_string(),
                    team: "red".to_string(),
                    frags: 16,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let server = GameServer::from(&generic);
        assert_eq!(server.players()[0].name(), "vikpe".to_string());
        assert_eq!(server.players()[1].name(), "XantoM".to_string());
        assert_eq!(server.players()[2].name(), "abc".to_string());
        assert_eq!(server.teams()[0].name(), "red".to_string());
        assert_eq!(server.teams()[1].name(), "blue".to_string());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization() -> anyhow::Result<()> {
        let server = GameServer {
            software_type: SoftwareType::Mvdsv,
            ip: "10.10.10.10".to_string(),
            port: 28000,
            settings: GameServerSettings {
                maxclients: 8,
                maxspectators: 4,
                ..GameServerSettings::default()
            },
            teams: vec![],
            players: vec![],
            spectators: vec![],
            qtv_stream: None,
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                country_name: Some("United States".to_string()),
                city: Some("New York".to_string()),
                region: Some("North America".to_string()),
                coords: Some(Coords::new(40.7128, -74.0060)),
            },
        };

        let server_json = r#"{"server_type":"game_server","software_type":"mvdsv","address":"10.10.10.10:28000","ip":"10.10.10.10","port":28000,"mode":"ffa","settings":{"admin":null,"broadcast":null,"city":null,"coords":null,"countrycode":null,"deathmatch":0,"epoch":null,"fraglimit":0,"gamedir":"","hostname":"","hostport":null,"ktxver":null,"map":"","matchtag":null,"maxclients":8,"maxspectators":4,"mode":null,"needpass":0,"serverdemo":null,"status":null,"sv_antilag":null,"teamplay":0,"timelimit":0,"version":""},"client_count":0,"client_limit":8,"teams":[],"players":[],"spectators":[],"total_spectator_count":0,"qtv_stream":null,"geo":{"country_code":"US","country_name":"United States","city":"New York","region":"North America","coords":{"lat":40.7128,"lng":-74.006}},"score":0}"#;

        // ensure round-trip serialization
        assert_eq!(serde_json::to_string(&server)?, server_json);
        assert_eq!(serde_json::from_str::<GameServer>(server_json)?, server);

        Ok(())
    }
}
