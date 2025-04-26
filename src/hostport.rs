//! Struct representing a host and port combination
use anyhow::Result;
use std::fmt::Display;
use std::net::Ipv4Addr;
use thiserror::Error;

#[cfg(feature = "json")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct HostPort {
    host: String,
    port: u16,
}

impl HostPort {
    pub fn new(host: String, port: u16) -> Result<HostPort, HostPortError> {
        if !is_valid_host(&host) {
            return Err(HostPortError::InvalidHost(host));
        }
        Ok(Self { host, port })
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

fn is_valid_host(value: &str) -> bool {
    is_valid_domain(value) || is_valid_ip(value)
}

fn is_valid_ip(value: &str) -> bool {
    value.parse::<Ipv4Addr>().is_ok()
}

fn is_valid_domain(value: &str) -> bool {
    if value.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    !value.is_empty()
        && value.split('.').all(|part| {
            part.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        })
        && !value.starts_with('.')
        && !value.ends_with('.')
}

impl TryFrom<&str> for HostPort {
    type Error = HostPortError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (host, port_str) = value.split_once(':').ok_or(HostPortError::InvalidFormat)?;
        let port = port_str.parse::<u16>()?;
        HostPort::new(host.to_string(), port)
    }
}

impl Display for HostPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

#[cfg(feature = "json")]
impl Serialize for HostPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "json")]
impl<'de> Deserialize<'de> for HostPort {
    fn deserialize<D>(deserializer: D) -> Result<HostPort, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_value = String::deserialize(deserializer)?;
        HostPort::try_from(string_value.as_str()).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum HostPortError {
    #[error("Invalid host: {0}")]
    InvalidHost(String),

    #[error("Invalid format, expected host:port")]
    InvalidFormat,

    #[error("Failed to parse host: {0}")]
    HostParseError(String),

    #[error("Failed to parse port: {0}")]
    PortParseError(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_valid_host() {
        assert!(is_valid_host("localhost"));
        assert!(is_valid_host("quake.se"));
        assert!(is_valid_host("quake-world.se"));
        assert!(is_valid_host("10.10.10.10"));
        assert!(is_valid_host("a10"));
        assert!(is_valid_host("10a"));

        assert!(!is_valid_host(""));
        assert!(!is_valid_host("aaa."));
        assert!(!is_valid_host(".aaa"));
        assert!(!is_valid_host("0"));
        assert!(!is_valid_host("10"));
    }

    #[test]
    fn test_new() -> Result<()> {
        let hostport = HostPort::new("quake.se".to_string(), 28501)?;
        assert_eq!(hostport.host(), "quake.se");
        assert_eq!(hostport.port(), 28501);
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<()> {
        assert_eq!(
            HostPort::try_from("quake.se"),
            Err(HostPortError::InvalidFormat)
        );
        assert!(matches!(
            HostPort::try_from("quake.se:aaa"),
            Err(HostPortError::PortParseError(_))
        ));
        assert_eq!(HostPort::try_from("quake.se:28501")?, {
            HostPort {
                host: "quake.se".to_string(),
                port: 28501,
            }
        });
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let hostport = HostPort::new("quake.se".to_string(), 28501)?;
        assert_eq!(hostport.to_string(), "quake.se:28501");
        Ok(())
    }

    #[test]
    fn test_serialize() -> Result<()> {
        let hostport = HostPort::new("quake.se".to_string(), 28501)?;
        assert_eq!(
            serde_json::to_string(&hostport)?,
            r#""quake.se:28501""#.to_string(),
        );
        Ok(())
    }

    #[test]
    fn test_deserialize() -> Result<()> {
        assert_eq!(
            serde_json::from_str::<HostPort>(r#""quake.se:28501""#)?,
            HostPort {
                host: "quake.se".to_string(),
                port: 28501,
            }
        );
        Ok(())
    }
}
