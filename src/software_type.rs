use std::fmt::Display;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "json",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum SoftwareType {
    FortressOne,
    Fte,
    Mvdsv,
    Qtv,
    Qwfwd,
    Unknown,
}

impl Display for SoftwareType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoftwareType::FortressOne => write!(f, "FortressOne"),
            SoftwareType::Fte => write!(f, "FTE"),
            SoftwareType::Mvdsv => write!(f, "MVDSV"),
            SoftwareType::Qtv => write!(f, "QTV"),
            SoftwareType::Qwfwd => write!(f, "QWFWD"),
            SoftwareType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl SoftwareType {
    pub fn from_version(version: &str) -> Self {
        let prefix = version.split_once(' ').map(|(v, _)| v).unwrap_or(version);

        match prefix.to_lowercase().as_str() {
            "fo" => SoftwareType::FortressOne,
            "fte" => SoftwareType::Fte,
            "mvdsv" => SoftwareType::Mvdsv,
            "qtvgo" => SoftwareType::Qtv,
            "qtv" => SoftwareType::Qtv,
            "qwfwd" => SoftwareType::Qwfwd,
            _ => SoftwareType::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(SoftwareType::FortressOne.to_string(), "FortressOne");
        assert_eq!(SoftwareType::Fte.to_string(), "FTE");
        assert_eq!(SoftwareType::Mvdsv.to_string(), "MVDSV");
        assert_eq!(SoftwareType::Qtv.to_string(), "QTV");
        assert_eq!(SoftwareType::Qwfwd.to_string(), "QWFWD");
        assert_eq!(SoftwareType::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_from_version() {
        assert_eq!(
            SoftwareType::from_version("fo     1.0"),
            SoftwareType::FortressOne
        );
        assert_eq!(SoftwareType::from_version("fte 1.0"), SoftwareType::Fte);
        assert_eq!(SoftwareType::from_version("mvdsv 1.0"), SoftwareType::Mvdsv);
        assert_eq!(SoftwareType::from_version("qtvgo 1.0"), SoftwareType::Qtv);
        assert_eq!(SoftwareType::from_version("qtv 1.0"), SoftwareType::Qtv);
        assert_eq!(SoftwareType::from_version("qwfwd 1.0"), SoftwareType::Qwfwd);
        assert_eq!(
            SoftwareType::from_version("unknown 1.0"),
            SoftwareType::Unknown
        );
    }
}
