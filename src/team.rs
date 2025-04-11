use crate::gameserver::Player;
use quake_text::unicode;
use std::cmp::Ordering;
use std::collections::HashMap;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Team {
    pub name: String,
    pub frags: i32,
    pub ping: u32,
    pub top_color: u8,
    pub bottom_color: u8,
}

#[derive(Default)]
#[allow(dead_code)]
struct TempTeam {
    name: String,
    frags: i32,
    ping_sum: f32,
    player_count: usize,
    colors: Vec<(u8, u8)>,
}

pub fn from_players(players: &[Player]) -> Vec<Team> {
    let mut temp: HashMap<String, TempTeam> = HashMap::new();

    for player in players {
        let team = temp.entry(player.team.clone()).or_default();
        team.name = player.team.clone();
        team.frags += player.frags;
        team.ping_sum += player.ping as f32;
        team.player_count += 1;
        team.colors.push((player.top_color, player.bottom_color));
    }

    let mut teams: Vec<Team> = Vec::new();
    for team in temp.values() {
        let (top_color, bottom_color) = get_majority_color(&team.colors);
        teams.push(Team {
            name: team.name.clone(),
            frags: team.frags,
            ping: (team.ping_sum / team.player_count as f32).round() as u32,
            top_color,
            bottom_color,
        });
    }
    teams.sort();
    teams
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> Ordering {
        unicode::ord(&self.name, &other.name)
    }
}

fn get_majority_color(colors: &[(u8, u8)]) -> (u8, u8) {
    if colors.is_empty() {
        return (0, 0);
    } else if colors.len() < 3 {
        return colors[0];
    }

    let mut color_count: HashMap<(u8, u8), usize> = HashMap::new();

    for color in colors {
        let count = color_count.entry(*color).or_default();
        *count += 1;
    }

    let mut max_count = 0;
    let mut majority_color = (0, 0);

    for (color, count) in color_count {
        if count > max_count {
            max_count = count;
            majority_color = color;
        }
    }

    majority_color
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_clients() -> Result<()> {
        let clients = vec![
            Player {
                team: "red".to_string(),
                frags: 10,
                ping: 12,
                top_color: 0,
                bottom_color: 0,
                ..Default::default()
            },
            Player {
                team: "red".to_string(),
                frags: 5,
                ping: 25,
                top_color: 4,
                bottom_color: 4,
                ..Default::default()
            },
            Player {
                team: "red".to_string(),
                frags: 2,
                ping: 25,
                top_color: 4,
                bottom_color: 4,
                ..Default::default()
            },
            Player {
                team: "blue".to_string(),
                frags: 7,
                ping: 52,
                top_color: 13,
                bottom_color: 13,
                ..Default::default()
            },
        ];

        let teams = from_players(&clients);
        assert_eq!(teams.len(), 2);

        assert_eq!(
            teams[0],
            Team {
                name: "blue".to_string(),
                frags: 7,
                ping: 52,
                top_color: 13,
                bottom_color: 13,
            }
        );

        assert_eq!(
            teams[1],
            Team {
                name: "red".to_string(),
                frags: 17,
                ping: 21,
                top_color: 4,
                bottom_color: 4,
            }
        );

        Ok(())
    }

    #[test]
    fn test_get_majority_color() {
        let m = get_majority_color;
        assert_eq!(m(&[]), (0, 0));
        assert_eq!(m(&[(1, 1)]), (1, 1));
        assert_eq!(m(&[(1, 1), (0, 0)]), (1, 1));
        assert_eq!(m(&[(0, 0), (1, 1), (1, 1)]), (1, 1));
    }
}
