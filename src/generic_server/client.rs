//! Generic client connected to a server (client, player, spectator)
use crate::net::tokenize::tokenize;
use anyhow::{Result, anyhow as e};
use quake_text::{bytestr, unicode};
use std::cmp::Ordering;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const PLAYER_MIN_PING: usize = 12;
const PLAYER_MAX_PING: usize = 600;

/// A client connected to a [`GenericServer`](crate::GenericServer).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenericClient {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) team: String,
    pub(crate) frags: i32,
    pub(crate) ping: u32,
    pub(crate) time: u32,
    pub(crate) top_color: u8,
    pub(crate) bottom_color: u8,
    pub(crate) skin: String,
    pub(crate) auth_cc: String,
    pub(crate) is_spectator: bool,
    pub(crate) is_bot: bool,
}

impl TryFrom<&[u8]> for GenericClient {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let tokens = tokenize(bytestr::to_unicode(bytes).as_str());

        if tokens.len() < 8 {
            return Err(e!("Invalid token count"));
        }

        let id: u32 = tokens[0].parse()?;
        let mut frags: i32 = tokens[1].parse()?;
        let time: u32 = tokens[2].parse()?;
        let ping_: i32 = tokens[3].parse()?;
        let mut name = tokens[4].to_string();
        let skin = tokens[5].to_string();
        let top_color: u8 = tokens[6].parse()?;
        let bottom_color: u8 = tokens[7].parse()?;
        let team = match tokens.len() >= 9 {
            true => tokens[8].to_string(),
            _ => "".to_string(),
        };
        let auth_cc = match tokens.len() >= 10 {
            true => tokens[9].to_string(),
            _ => "".to_string(),
        };
        let is_spectator = ping_ < 1;
        if is_spectator {
            frags = 0;
            name = name.trim_start_matches("\\s\\").to_string();
        }
        let ping = ping_.unsigned_abs();

        let is_bot = match tokens.len() >= 11 {
            true => tokens[10] == "b",
            false => !(PLAYER_MIN_PING..=PLAYER_MAX_PING).contains(&(ping as usize)),
        };

        Ok(Self {
            id,
            name,
            team,
            frags,
            ping,
            time,
            top_color,
            bottom_color,
            skin,
            auth_cc,
            is_spectator,
            is_bot,
        })
    }
}

impl PartialOrd for GenericClient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GenericClient {
    fn cmp(&self, other: &Self) -> Ordering {
        unicode::ord(&self.name, &other.name)
    }
}

impl GenericClient {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn team(&self) -> &str {
        &self.team
    }

    pub fn frags(&self) -> i32 {
        self.frags
    }

    pub fn ping(&self) -> u32 {
        self.ping
    }

    pub fn time(&self) -> u32 {
        self.time
    }

    pub fn top_color(&self) -> u8 {
        self.top_color
    }

    pub fn bottom_color(&self) -> u8 {
        self.bottom_color
    }

    pub fn skin(&self) -> &str {
        &self.skin
    }

    pub fn auth_cc(&self) -> &str {
        &self.auth_cc
    }

    pub fn is_spectator(&self) -> bool {
        self.is_spectator
    }

    pub fn is_bot(&self) -> bool {
        self.is_bot
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_try_from_bytes() -> Result<()> {
        // invalid input
        assert!(GenericClient::try_from(br#"a 0 0 0 "name" "" 4 4 "sk" """#.as_slice()).is_err()); // invalid id
        assert!(GenericClient::try_from(br#"0 a 0 0 "name" "" 4 4 "sk" """#.as_slice()).is_err()); // invalid frags
        assert!(GenericClient::try_from(br#"0 0 a 0 "name" "" 4 4 "sk" """#.as_slice()).is_err()); // invalid time
        assert!(GenericClient::try_from(br#"0 0 0 a "name" "" 4 4 "sk" """#.as_slice()).is_err()); // invalid ping
        assert!(GenericClient::try_from(br#"0 0 0 0 "name" "" a 4 "sk" """#.as_slice()).is_err()); // invalid top_color
        assert!(GenericClient::try_from(br#"0 0 0 0 "name" "" 4 a "sk" """#.as_slice()).is_err()); // invalid bottom_color

        // player
        {
            let bytes = br#"63 43 41 25 "Player" "" 4 4 "red" """#;
            let client = GenericClient::try_from(bytes.as_slice())?;
            assert_eq!(
                client,
                GenericClient {
                    id: 63,
                    name: "Player".to_string(),
                    team: "red".to_string(),
                    frags: 43,
                    ping: 25,
                    time: 41,
                    top_color: 4,
                    bottom_color: 4,
                    skin: "".to_string(),
                    auth_cc: "".to_string(),
                    is_spectator: false,
                    is_bot: false,
                }
            );
            assert!(!client.is_spectator())
        }

        // spectator
        {
            let bytes = br#"74 -9999 3 -33 "\s\ razor" "8" 3 11 "sr" """#.as_slice();
            let client = GenericClient::try_from(bytes)?;
            assert_eq!(
                client,
                GenericClient {
                    id: 74,
                    name: " razor".to_string(),
                    team: "sr".to_string(),
                    frags: 0,
                    ping: 33,
                    time: 3,
                    top_color: 3,
                    bottom_color: 11,
                    skin: "8".to_string(),
                    auth_cc: "".to_string(),
                    is_spectator: true,
                    is_bot: false,
                }
            );
            assert!(client.is_spectator())
        }

        // qtv/qwfwd client
        {
            let bytes = br#"1446 0 32 64 "Zepp" "" 0 0"#;
            let client = GenericClient::try_from(bytes.as_slice())?;
            assert_eq!(
                client,
                GenericClient {
                    id: 1446,
                    name: "Zepp".to_string(),
                    team: "".to_string(),
                    frags: 0,
                    ping: 64,
                    time: 32,
                    top_color: 0,
                    bottom_color: 0,
                    skin: "".to_string(),
                    auth_cc: "".to_string(),
                    is_spectator: false,
                    is_bot: false,
                }
            );
        }
        Ok(())
    }

    #[test]
    fn test_cmp() {
        let mut clients = vec![
            GenericClient {
                name: "foo".to_string(),
                ..Default::default()
            },
            GenericClient {
                name: "áøå2".to_string(),
                ..Default::default()
            },
            GenericClient {
                name: "axe".to_string(),
                ..Default::default()
            },
            GenericClient {
                name: "B".to_string(),
                ..Default::default()
            },
        ];
        clients.sort();
        assert_eq!(clients[0].name, "axe");
        assert_eq!(clients[1].name, "áøå2");
        assert_eq!(clients[2].name, "B");
        assert_eq!(clients[3].name, "foo");
    }
}
