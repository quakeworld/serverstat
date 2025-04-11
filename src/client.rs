use crate::tokenize;
use anyhow::Result;
use quake_text::{bytestr, unicode};

use std::cmp::Ordering;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

const PLAYER_MIN_PING: usize = 12;
const PLAYER_MAX_PING: usize = 600;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct QuakeClient {
    pub id: u32,
    pub name: String,
    pub team: String,
    pub frags: i32,
    pub ping: u32,
    pub time: u32,
    pub top_color: u8,
    pub bottom_color: u8,
    pub skin: String,
    pub auth_cc: String,
    pub is_spectator: bool,
    pub is_bot: bool,
}

impl TryFrom<&[u8]> for QuakeClient {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let parts: Vec<String> = tokenize::tokenize(bytestr::to_unicode(bytes).as_str());
        let id: u32 = parts[0].parse()?;
        let mut frags: i32 = parts[1].parse()?;
        let time: u32 = parts[2].parse()?;
        let ping_: i32 = parts[3].parse()?;
        let mut name = parts[4].to_string();
        let skin = parts[5].to_string();
        let top_color: u8 = parts[6].parse()?;
        let bottom_color: u8 = parts[7].parse()?;
        let team = match parts.len() >= 9 {
            true => parts[8].to_string(),
            _ => "".to_string(),
        };
        let auth_cc = match parts.len() >= 10 {
            true => parts[9].to_string(),
            _ => "".to_string(),
        };
        let is_spectator = ping_ < 1;
        if is_spectator {
            frags = 0;
            name = name.trim_start_matches("\\s\\").to_string();
        }
        let ping = ping_.unsigned_abs();
        let is_bot = !(PLAYER_MIN_PING..=PLAYER_MAX_PING).contains(&(ping as usize));

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

impl PartialOrd for QuakeClient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QuakeClient {
    fn cmp(&self, other: &Self) -> Ordering {
        unicode::ord(&self.name, &other.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_try_from_bytes() -> Result<()> {
        // player
        {
            let bytes = br#"63 43 41 25 "ToT_Oddjob" "" 4 4 "red" """#;
            let client = QuakeClient::try_from(bytes.as_slice())?;

            assert_eq!(
                client,
                QuakeClient {
                    id: 63,
                    name: "ToT_Oddjob".to_string(),
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
        }

        // spectator
        {
            let bytes = br#"74 -9999 3 -33 "\s\ razor" "8" 3 11 "sr" """#;
            let client = QuakeClient::try_from(bytes.as_slice())?;

            assert_eq!(
                client,
                QuakeClient {
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
            )
        }

        // qtv/qwfwd client
        {
            let bytes = br#"1446 0 32 64 "Zepp" "" 0 0"#;
            let client = QuakeClient::try_from(bytes.as_slice())?;
            assert_eq!(
                client,
                QuakeClient {
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
            QuakeClient {
                name: "foo".to_string(),
                ..Default::default()
            },
            QuakeClient {
                name: "áøå2".to_string(),
                ..Default::default()
            },
            QuakeClient {
                name: "axe".to_string(),
                ..Default::default()
            },
            QuakeClient {
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
