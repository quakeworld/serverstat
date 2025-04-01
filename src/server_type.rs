use std::fmt::Display;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "json",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum ServerType {
    GameServer,
    ProxyServer,
    QtvServer,
    Unknown,
}

impl Display for ServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerType::GameServer => write!(f, "GameServer"),
            ServerType::ProxyServer => write!(f, "ProxyServer"),
            ServerType::QtvServer => write!(f, "QtvServer"),
            ServerType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl ServerType {
    pub fn from_version(version: &str) -> Self {
        let prefix = version
            .split_once(' ')
            .map(|(v, _)| v)
            .unwrap_or(version)
            .to_lowercase();

        if ["fo", "fte", "mvdsv"].contains(&prefix.as_str()) {
            ServerType::GameServer
        } else if ["qtvgo", "qtv"].contains(&prefix.as_str()) {
            ServerType::QtvServer
        } else if ["qwfwd"].contains(&prefix.as_str()) {
            ServerType::ProxyServer
        } else {
            ServerType::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_version() {
        assert_eq!(
            ServerType::from_version("fo     1.0"),
            ServerType::GameServer
        );
        assert_eq!(ServerType::from_version("fte 1.0"), ServerType::GameServer);
        assert_eq!(
            ServerType::from_version("mvdsv 1.0"),
            ServerType::GameServer
        );
        assert_eq!(ServerType::from_version("qtvgo 1.0"), ServerType::QtvServer);
        assert_eq!(ServerType::from_version("qtv 1.0"), ServerType::QtvServer);
        assert_eq!(
            ServerType::from_version("qwfwd 1.0"),
            ServerType::ProxyServer
        );
        assert_eq!(ServerType::from_version("unknown 1.0"), ServerType::Unknown);
    }
}
