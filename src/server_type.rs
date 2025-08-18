use std::fmt::Display;

/// Server types: game, proxy, and QTV server
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum ServerType {
    GameServer,
    ProxyServer,
    QtvServer,
    GenericServer,
}

impl Default for ServerType {
    fn default() -> Self {
        Self::GenericServer
    }
}

impl Display for ServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerType::GameServer => write!(f, "game_server"),
            ServerType::ProxyServer => write!(f, "proxy_server"),
            ServerType::QtvServer => write!(f, "qtv_server"),
            ServerType::GenericServer => write!(f, "generic_server"),
        }
    }
}

impl ServerType {
    pub fn from_version(version: &str) -> Self {
        let version_ = version.trim().to_lowercase();
        let version_str = version_.as_str();
        let prefix = version_str
            .split_once(' ')
            .map(|(v, _)| v)
            .unwrap_or(version_str);

        if ["fo", "fte", "mvdsv", "zquake"].contains(&prefix) {
            return ServerType::GameServer;
        } else if ["qtvgo", "qtv"].contains(&prefix) {
            return ServerType::QtvServer;
        } else if ["qwfwd"].contains(&prefix) {
            return ServerType::ProxyServer;
        }

        // misc game servers
        for name in ["kkqwsv", "cpqwsv"] {
            if version_str.contains(name) {
                return ServerType::GameServer;
            }
        }

        ServerType::GenericServer
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(ServerType::GameServer.to_string(), "game_server");
        assert_eq!(ServerType::ProxyServer.to_string(), "proxy_server");
        assert_eq!(ServerType::QtvServer.to_string(), "qtv_server");
        assert_eq!(ServerType::GenericServer.to_string(), "generic_server");
    }

    #[test]
    fn test_from_version() {
        assert_eq!(
            ServerType::from_version("2.41-CPQWSV win32"),
            ServerType::GameServer,
        );
        assert_eq!(
            ServerType::from_version("fo     1.0"),
            ServerType::GameServer
        );
        assert_eq!(ServerType::from_version("fte 1.0"), ServerType::GameServer);
        assert_eq!(
            ServerType::from_version("MVDSV 1.0"),
            ServerType::GameServer
        );
        assert_eq!(
            ServerType::from_version("zquake 1.0"),
            ServerType::GameServer
        );
        assert_eq!(ServerType::from_version("qtvgo 1.0"), ServerType::QtvServer);
        assert_eq!(ServerType::from_version("qtv 1.0"), ServerType::QtvServer);
        assert_eq!(
            ServerType::from_version("qwfwd 1.0"),
            ServerType::ProxyServer
        );
        assert_eq!(
            ServerType::from_version("unknown 1.0"),
            ServerType::GenericServer
        );
    }
}
