use quake_serverinfo::Settings;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Configuration settings for [`ProxyServer`](crate::ProxyServer) instances.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProxySettings {
    hostname: String,
    maxclients: u32,
    version: String,
    city: Option<String>,
    coords: Option<String>,
    countrycode: Option<String>,
    hostport: Option<String>,
}

#[allow(dead_code)]
impl ProxySettings {
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn maxclients(&self) -> u32 {
        self.maxclients
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn city(&self) -> Option<&str> {
        self.city.as_deref()
    }

    pub fn coords(&self) -> Option<&str> {
        self.coords.as_deref()
    }

    pub fn countrycode(&self) -> Option<&str> {
        self.countrycode.as_deref()
    }

    pub fn hostport(&self) -> Option<&str> {
        self.hostport.as_deref()
    }
}

impl From<&Settings> for ProxySettings {
    fn from(settings: &Settings) -> Self {
        Self {
            hostname: settings.hostname.clone().unwrap_or_default(),
            maxclients: settings.maxclients.unwrap_or_default() as u32,
            version: settings.version.clone().unwrap_or_default(),
            city: settings.city.clone(),
            coords: settings.coords.clone(),
            countrycode: settings.countrycode.clone(),
            hostport: settings.hostport.clone(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_proxysettings_from_settings() {
        let proxy_settings = ProxySettings::from(&Settings {
            hostname: Some("Test Server".to_string()),
            maxclients: Some(16),
            version: Some("QWFWD 1.0".to_string()),
            city: Some("Test City".to_string()),
            coords: Some("12.34,56.78".to_string()),
            countrycode: Some("TC".to_string()),
            hostport: Some("test.server:28000".to_string()),
            ..Default::default()
        });
        assert_eq!(proxy_settings.hostname(), "Test Server");
        assert_eq!(proxy_settings.maxclients(), 16);
        assert_eq!(proxy_settings.version(), "QWFWD 1.0");
        assert_eq!(proxy_settings.city(), Some("Test City"));
        assert_eq!(proxy_settings.coords(), Some("12.34,56.78"));
        assert_eq!(proxy_settings.countrycode(), Some("TC"));
        assert_eq!(proxy_settings.hostport(), Some("test.server:28000"));
    }
}
