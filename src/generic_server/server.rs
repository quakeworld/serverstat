use crate::{GenericClient, GeoInfo, QtvStream, ServerType, SoftwareType};
pub use quake_serverinfo::Settings;

#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

/// Represents a generic server.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct GenericServer {
    pub(crate) software_type: SoftwareType,
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) settings: Settings,
    pub(crate) clients: Vec<GenericClient>,
    pub(crate) qtv_stream: Option<QtvStream>,
    pub(crate) geo: GeoInfo,
}

#[allow(dead_code)]
impl GenericServer {
    pub fn server_type(&self) -> ServerType {
        ServerType::GenericServer
    }

    pub fn software_type(&self) -> SoftwareType {
        self.software_type.clone()
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.ip(), self.port())
    }

    pub fn ip(&self) -> &str {
        &self.ip
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn clients(&self) -> &[GenericClient] {
        &self.clients
    }

    pub fn players(&self) -> impl Iterator<Item = &GenericClient> {
        self.clients()
            .iter()
            .filter(|client| !client.is_spectator())
    }

    pub fn spectators(&self) -> impl Iterator<Item = &GenericClient> {
        self.clients().iter().filter(|client| client.is_spectator())
    }

    pub fn qtv_stream(&self) -> Option<&QtvStream> {
        self.qtv_stream.as_ref()
    }

    pub fn geo(&self) -> &GeoInfo {
        &self.geo
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for GenericServer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("GenericServer", 11)?;
        state.serialize_field("server_type", &self.server_type())?;
        state.serialize_field("software_type", &self.software_type())?;
        state.serialize_field("address", &self.address())?;
        state.serialize_field("ip", self.ip())?;
        state.serialize_field("port", &self.port())?;
        state.serialize_field("settings", self.settings())?;
        state.serialize_field("client_count", &self.clients().len())?;
        state.serialize_field("client_limit", &self.settings().maxclients)?;
        state.serialize_field("clients", self.clients())?;
        state.serialize_field("qtv_stream", &self.qtv_stream())?;
        state.serialize_field("geo", self.geo())?;
        state.end()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::{Coords, GenericClient, GeoInfo, SoftwareType};
    use pretty_assertions::assert_eq;
    use quake_serverinfo::Settings;

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization() -> anyhow::Result<()> {
        let server = GenericServer {
            software_type: SoftwareType::Unknown,
            ip: "10.10.10.10".to_string(),
            port: 28501,
            settings: Settings {
                hostname: Some("Random Server".to_string()),
                maxclients: Some(128),
                ..Default::default()
            },
            clients: vec![
                GenericClient {
                    name: "XantoM".to_string(),
                    ..GenericClient::default()
                },
                GenericClient {
                    name: "vikpe".to_string(),
                    ..GenericClient::default()
                },
            ],
            qtv_stream: None,
            geo: GeoInfo {
                country_code: Some("US".to_string()),
                country_name: Some("United States".to_string()),
                city: Some("New York".to_string()),
                region: Some("North America".to_string()),
                coords: Some(Coords::new(40.7128, -74.0060)),
            },
        };

        let server_json = r#"{"server_type":"generic_server","software_type":"unknown","address":"10.10.10.10:28501","ip":"10.10.10.10","port":28501,"settings":{"admin":null,"broadcast":null,"city":null,"coords":null,"countrycode":null,"deathmatch":null,"epoch":null,"fpd":null,"fraglimit":null,"gamedir":null,"hostname":"Random Server","hostport":null,"ktxmode":null,"ktxver":null,"map":null,"matchtag":null,"maxclients":128,"maxfps":null,"maxspectators":null,"mode":null,"needpass":null,"pm_ktjump":null,"progs":null,"qvm":null,"serverdemo":null,"status":null,"sv_antilag":null,"teamplay":null,"timelimit":null,"version":null,"z_ext":null},"client_count":2,"client_limit":128,"clients":[{"id":0,"name":"XantoM","team":"","frags":0,"ping":0,"time":0,"top_color":0,"bottom_color":0,"skin":"","auth_cc":"","is_spectator":false,"is_bot":false},{"id":0,"name":"vikpe","team":"","frags":0,"ping":0,"time":0,"top_color":0,"bottom_color":0,"skin":"","auth_cc":"","is_spectator":false,"is_bot":false}],"qtv_stream":null,"geo":{"country_code":"US","country_name":"United States","city":"New York","region":"North America","coords":{"lat":40.7128,"lng":-74.006}}}"#;

        // ensure round-trip serialization
        assert_eq!(serde_json::to_string(&server)?, server_json);
        assert_eq!(serde_json::from_str::<GenericServer>(server_json)?, server);

        Ok(())
    }
}
