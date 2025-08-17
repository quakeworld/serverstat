use crate::{GameServer, GenericServer, GeoInfo, ProxyServer, QtvServer, ServerType, SoftwareType};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[serde(tag = "server_type")]
pub enum Server {
    #[serde(rename = "game_server")]
    Game(GameServer),

    #[serde(rename = "proxy_server")]
    Proxy(ProxyServer),

    #[serde(rename = "qtv_server")]
    Qtv(QtvServer),

    #[serde(rename = "generic_server")]
    Generic(GenericServer),
}

impl Server {
    pub fn server_type(&self) -> ServerType {
        match self {
            Server::Game(_) => ServerType::GameServer,
            Server::Proxy(_) => ServerType::ProxyServer,
            Server::Qtv(_) => ServerType::QtvServer,
            Server::Generic(_) => ServerType::Unknown,
        }
    }

    pub fn software_type(&self) -> SoftwareType {
        match self {
            Server::Game(s) => s.software_type(),
            Server::Proxy(s) => s.software_type(),
            Server::Qtv(s) => s.software_type(),
            Server::Generic(s) => s.software_type(),
        }
    }

    pub fn ip(&self) -> &str {
        match self {
            Server::Game(s) => s.ip(),
            Server::Proxy(s) => s.ip(),
            Server::Qtv(s) => s.ip(),
            Server::Generic(s) => s.ip(),
        }
    }

    pub fn port(&self) -> u16 {
        match self {
            Server::Game(s) => s.port(),
            Server::Proxy(s) => s.port(),
            Server::Qtv(s) => s.port(),
            Server::Generic(s) => s.port(),
        }
    }

    pub fn address(&self) -> String {
        match self {
            Server::Game(s) => s.address(),
            Server::Proxy(s) => s.address(),
            Server::Qtv(s) => s.address(),
            Server::Generic(s) => s.address(),
        }
    }

    pub fn geo(&self) -> &GeoInfo {
        match self {
            Server::Game(s) => s.geo(),
            Server::Proxy(s) => s.geo(),
            Server::Qtv(s) => s.geo(),
            Server::Generic(s) => s.geo(),
        }
    }
}
