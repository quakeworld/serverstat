use super::team;
use crate::{GenericClient, GenericServer};

use crate::{ClientSlots, GeoInfo, Player, QtvStream, ServerType, SoftwareType, Spectator, Team};
use quake_serverinfo::Settings;
use quake_text::unicode;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a server where clients connect as [`Player`] or [`Spectator`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
