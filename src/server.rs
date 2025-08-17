use crate::{GameServer, GenericServer, GeoInfo, ProxyServer, QtvServer, ServerType, SoftwareType};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum Server {
    Game(GameServer),
    Proxy(ProxyServer),
    Qtv(QtvServer),
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

#[cfg(feature = "serde")]
impl serde::Serialize for Server {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Server::Game(s) => s.serialize(serializer),
            Server::Proxy(s) => s.serialize(serializer),
            Server::Qtv(s) => s.serialize(serializer),
            Server::Generic(s) => s.serialize(serializer),
        }
    }
}
