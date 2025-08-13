use quake_serverinfo::Settings;

/// Configuration settings for [`GameServer`](crate::GameServer) instances.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameServerSettings {
    pub(super) admin: Option<String>,
    pub(super) broadcast: Option<i32>,
    pub(super) city: Option<String>,
    pub(super) coords: Option<String>,
    pub(super) countrycode: Option<String>,
    pub(super) deathmatch: u32,
    pub(super) epoch: Option<i32>,
    pub(super) fraglimit: u32,
    pub(super) gamedir: String,
    pub(super) hostname: String,
    pub(super) hostport: Option<String>,
    pub(super) ktxver: Option<String>,
    pub(super) map: String,
    pub(super) matchtag: Option<String>,
    pub(super) maxclients: u32,
    pub(super) maxspectators: u32,
    pub(super) mode: Option<String>,
    pub(super) needpass: u32,
    pub(super) serverdemo: Option<String>,
    pub(super) status: Option<String>,
    pub(super) sv_antilag: Option<i32>,
    pub(super) teamplay: u32,
    pub(super) timelimit: u32,
    pub(super) version: String,
    // does anyone care about these?
    // pub(super) fpd: Option<i32>,
    // pub(super) maxfps: Option<i32>,
    // pub(super) pm_ktjump: Option<i32>,
    // pub(super) progs: Option<String>,
    // pub(super) qvm: Option<String>,
    // pub(super) z_ext: Option<i32>,
}

#[allow(dead_code)]
impl GameServerSettings {
    pub fn admin(&self) -> Option<&str> {
        self.admin.as_deref()
    }

    pub fn broadcast(&self) -> Option<i32> {
        self.broadcast
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

    pub fn deathmatch(&self) -> u32 {
        self.deathmatch
    }

    pub fn epoch(&self) -> Option<i32> {
        self.epoch
    }

    pub fn fraglimit(&self) -> u32 {
        self.fraglimit
    }

    pub fn gamedir(&self) -> &str {
        &self.gamedir
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn hostport(&self) -> Option<&str> {
        self.hostport.as_deref()
    }

    pub fn ktxver(&self) -> Option<&str> {
        self.ktxver.as_deref()
    }

    pub fn map(&self) -> &str {
        &self.map
    }

    pub fn matchtag(&self) -> Option<&str> {
        self.matchtag.as_deref()
    }

    pub fn maxclients(&self) -> u32 {
        self.maxclients
    }

    pub fn maxspectators(&self) -> u32 {
        self.maxspectators
    }

    pub fn mode(&self) -> Option<&str> {
        self.mode.as_deref()
    }

    pub fn needpass(&self) -> u32 {
        self.needpass
    }

    pub fn serverdemo(&self) -> Option<&str> {
        self.serverdemo.as_deref()
    }

    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    pub fn sv_antilag(&self) -> Option<i32> {
        self.sv_antilag
    }

    pub fn teamplay(&self) -> u32 {
        self.teamplay
    }

    pub fn timelimit(&self) -> u32 {
        self.timelimit
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

impl From<&Settings> for GameServerSettings {
    fn from(settings: &Settings) -> Self {
        let s = settings.clone();

        Self {
            admin: s.admin,
            broadcast: s.broadcast,
            city: s.city,
            coords: s.coords,
            countrycode: s.countrycode,
            deathmatch: s.deathmatch.unwrap_or_default() as u32,
            epoch: s.epoch,
            fraglimit: s.fraglimit.unwrap_or_default() as u32,
            gamedir: s.gamedir.unwrap_or_default(),
            hostname: s.hostname.unwrap_or_default(),
            hostport: s.hostport,
            ktxver: s.ktxver,
            map: s.map.unwrap_or_default(),
            matchtag: s.matchtag,
            maxclients: s.maxclients.unwrap_or_default() as u32,
            maxspectators: s.maxspectators.unwrap_or_default() as u32,
            mode: s.mode,
            needpass: s.needpass.unwrap_or_default() as u32,
            serverdemo: s.serverdemo,
            status: s.status,
            sv_antilag: s.sv_antilag,
            teamplay: s.teamplay.unwrap_or_default() as u32,
            timelimit: s.timelimit.unwrap_or_default() as u32,
            version: s.version.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_gameserversettings_from_settings() {
        let gs_settings = GameServerSettings::from(&Settings {
            hostname: Some("Test Server".to_string()),
            maxclients: Some(16),
            version: Some("QTV 1.0".to_string()),
            ..Default::default()
        });
        assert_eq!(gs_settings.admin(), None);
        assert_eq!(gs_settings.deathmatch(), 0);
        assert_eq!(gs_settings.hostname(), "Test Server");
        assert_eq!(gs_settings.maxclients(), 16);
        assert_eq!(gs_settings.version(), "QTV 1.0");
    }
}
