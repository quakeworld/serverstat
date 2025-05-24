//! Team: a collection of Players
use crate::game_server::player::Player;
use quake_text::unicode;
use std::cmp::Ordering;
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Team {
    name: String,
    frags: i32,
    ping: u32,
    top_color: u8,
    bottom_color: u8,
}

#[allow(dead_code)]
impl Team {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn frags(&self) -> i32 {
        self.frags
    }

    pub fn ping(&self) -> u32 {
        self.ping
    }

    pub fn top_color(&self) -> u8 {
        self.top_color
    }

    pub fn bottom_color(&self) -> u8 {
        self.bottom_color
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
struct TempTeam {
    name: String,
    frags: i32,
    ping_sum: f32,
    player_count: usize,
    colors: Vec<(u8, u8)>,
}

pub fn players_to_teams(players: &[Player]) -> Vec<Team> {
    let mut temp: HashMap<String, TempTeam> = HashMap::new();

    for player in players {
        let team = temp.entry(player.team().to_string()).or_default();
        team.name = player.team().to_string();
        team.frags += player.frags();
        team.ping_sum += player.ping() as f32;
        team.player_count += 1;
        team.colors
            .push((player.top_color(), player.bottom_color()));
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
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_players_to_teams() -> Result<()> {
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

        let teams = players_to_teams(&clients);
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

        assert_eq!(teams[1].name(), "red");
        assert_eq!(teams[1].frags(), 17);
        assert_eq!(teams[1].ping(), 21);
        assert_eq!(teams[1].top_color(), 4);
        assert_eq!(teams[1].bottom_color(), 4);
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
