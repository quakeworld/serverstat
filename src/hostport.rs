use anyhow::{Result, anyhow as e};
use std::fmt::Display;

#[cfg(feature = "json")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Hostport {
    pub host: String,
    pub port: u16,
}

impl Hostport {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
}

impl TryFrom<&str> for Hostport {
    type Error = anyhow::Error;

    fn try_from(address: &str) -> Result<Self, Self::Error> {
        let (host, port_str) = address
            .split_once(':')
            .ok_or_else(|| e!("Invalid hostport format, expected host:port"))?;
        Ok(Self {
            host: host.to_string(),
            port: port_str.parse::<u16>()?,
        })
    }
}

impl Display for Hostport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

#[cfg(feature = "json")]
impl Serialize for Hostport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "json")]
impl<'de> Deserialize<'de> for Hostport {
    fn deserialize<D>(deserializer: D) -> Result<Hostport, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_value = String::deserialize(deserializer)?;
        Hostport::try_from(string_value.as_str()).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_try_from_str() -> Result<()> {
        assert_eq!(
            Hostport::try_from("quake.se").unwrap_err().to_string(),
            "Invalid hostport format, expected host:port"
        );
        assert_eq!(Hostport::try_from("quake.se:28501")?, {
            Hostport {
                host: "quake.se".to_string(),
                port: 28501,
            }
        });
        Ok(())
    }

    #[test]
    fn test_display() {
        let hostport = Hostport::new("quake.se".to_string(), 28501);
        assert_eq!(hostport.to_string(), "quake.se:28501");
    }

    #[test]
    fn test_serialize() -> Result<()> {
        let hostport = Hostport::new("quake.se".to_string(), 28501);
        assert_eq!(
            serde_json::to_string(&hostport)?,
            r#""quake.se:28501""#.to_string(),
        );
        Ok(())
    }

    #[test]
    fn test_deserialize() -> Result<()> {
        assert_eq!(
            serde_json::from_str::<Hostport>(r#""quake.se:28501""#)?,
            Hostport {
                host: "quake.se".to_string(),
                port: 28501,
            }
        );
        Ok(())
    }
}
