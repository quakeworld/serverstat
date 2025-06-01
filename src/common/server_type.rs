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
    Unknown,
}

impl Default for ServerType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for ServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerType::GameServer => write!(f, "game_server"),
            ServerType::ProxyServer => write!(f, "proxy_server"),
            ServerType::QtvServer => write!(f, "qtv_server"),
            ServerType::Unknown => write!(f, "unknown"),
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

        if ["fo", "fte", "mvdsv", "zquake"].contains(&prefix.as_str()) {
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
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(ServerType::GameServer.to_string(), "game_server");
        assert_eq!(ServerType::ProxyServer.to_string(), "proxy_server");
        assert_eq!(ServerType::QtvServer.to_string(), "qtv_server");
        assert_eq!(ServerType::Unknown.to_string(), "unknown");
    }

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
        assert_eq!(ServerType::from_version("unknown 1.0"), ServerType::Unknown);
    }
}
