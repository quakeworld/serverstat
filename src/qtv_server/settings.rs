use quake_serverinfo::Settings;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Configuration settings for [`QtvServer`](crate::QtvServer) instances.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QtvSettings {
    hostname: String,
    maxclients: u32,
    version: String,
}

#[allow(dead_code)]
impl QtvSettings {
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn maxclients(&self) -> u32 {
        self.maxclients
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

impl From<&Settings> for QtvSettings {
    fn from(settings: &Settings) -> Self {
        Self {
            hostname: settings.hostname.clone().unwrap_or_default(),
            maxclients: settings.maxclients.unwrap_or_default() as u32,
            version: settings.version.clone().unwrap_or_default(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_qtvsettings_from_settings() {
        let qtv_settings = QtvSettings::from(&Settings {
            hostname: Some("Test Server".to_string()),
            maxclients: Some(16),
            version: Some("QTV 1.0".to_string()),
            ..Default::default()
        });
        assert_eq!(qtv_settings.hostname(), "Test Server");
        assert_eq!(qtv_settings.maxclients(), 16);
        assert_eq!(qtv_settings.version(), "QTV 1.0");
    }
}
