use crate::{GameServer, GenericServer, ProxyServer, QtvServer, ServerType, SoftwareType};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Server {
    Game(GameServer),
    Generic(GenericServer),
    Proxy(ProxyServer),
    Qtv(QtvServer),
}

impl Server {
    fn server_type(&self) -> ServerType {
        match self {
            Server::Game(_) => ServerType::GameServer,
            Server::Generic(_) => ServerType::Unknown,
            Server::Proxy(_) => ServerType::ProxyServer,
            Server::Qtv(_) => ServerType::QtvServer,
        }
    }

    fn software_type(&self) -> SoftwareType {
        match self {
            Server::Game(s) => s.software_type(),
            Server::Generic(s) => s.software_type(),
            Server::Proxy(s) => s.software_type(),
            Server::Qtv(s) => s.software_type(),
        }
    }

    fn ip(&self) -> &str {
        match self {
            Server::Game(s) => s.ip(),
            Server::Generic(s) => s.ip(),
            Server::Proxy(s) => s.ip(),
            Server::Qtv(s) => s.ip(),
        }
    }

    fn port(&self) -> u16 {
        match self {
            Server::Game(s) => s.port(),
            Server::Generic(s) => s.port(),
            Server::Proxy(s) => s.port(),
            Server::Qtv(s) => s.port(),
        }
    }

    fn address(&self) -> &str {
        match self {
            Server::Game(s) => s.address(),
            Server::Generic(s) => s.address(),
            Server::Proxy(s) => s.address(),
            Server::Qtv(s) => s.address(),
        }
    }
}
