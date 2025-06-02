use crate::GameServer;

mod weight {
    pub const FULL_SERVER: f32 = 20.;
    pub const HUMAN_PLAYER: f32 = 4.;
    pub const SPECTATOR: f32 = 2.;
    pub const DMM4_FACTOR: f32 = 0.5;
}

pub(crate) fn from_game_server(server: &GameServer) -> u32 {
    calculate_score(Params::from_game_server(server))
}

/// returns a score based on activity
/// the score is floored to the closest multiple of 10
fn calculate_score(params: Params) -> u32 {
    let player_count = params.player_count as f32;

    if player_count < 2. {
        return (player_count * weight::HUMAN_PLAYER) as u32;
    }

    let player_limit = params.player_limit as f32;
    let spectator_count = params.spectator_count as f32;

    let server_score = {
        let fill_percentage = (player_count / player_limit).min(1.0);
        let score_factor = match params.deathmatch {
            Some(4) | None => weight::DMM4_FACTOR,
            _ => 1.0,
        };
        fill_percentage * weight::FULL_SERVER * score_factor
    };

    let player_score = player_count * weight::HUMAN_PLAYER;
    let spectator_score = spectator_count * weight::SPECTATOR;
    let exact_sum = server_score + player_score + spectator_score;

    // closest multiple of 10
    let floored_sum = (exact_sum / 10.).round() * 10.;
    floored_sum as u32
}

#[derive(Debug, PartialEq, Default)]
struct Params {
    player_count: u32,
    player_limit: u32,
    deathmatch: Option<i32>,
    spectator_count: u32,
}

impl Params {
    pub fn from_game_server(server: &GameServer) -> Self {
        let spectator_count = {
            let qtv_spectator_count = server.qtv_stream().map_or(0, |q| q.client_count());
            server.spectators().len() as u32 + qtv_spectator_count
        };

        Self {
            player_count: server.players().iter().filter(|p| !p.is_bot()).count() as u32,
            player_limit: server.settings().maxclients.unwrap_or(8) as u32,
            deathmatch: server.settings().deathmatch,
            spectator_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_game_server() {
        assert_eq!(
            calculate_score(Params {
                player_count: 0,
                ..Default::default()
            }),
            0
        );

        assert_eq!(
            calculate_score(Params {
                player_count: 1,
                ..Default::default()
            }),
            4
        );

        assert_eq!(
            calculate_score(Params {
                player_count: 2,
                player_limit: 4,
                ..Default::default()
            }),
            10
        );

        assert_eq!(
            calculate_score(Params {
                player_count: 2,
                player_limit: 2,
                ..Default::default()
            }),
            20
        );

        assert_eq!(
            calculate_score(Params {
                player_count: 2,
                player_limit: 2,
                deathmatch: Some(3),
                ..Default::default()
            }),
            30
        );

        assert_eq!(
            calculate_score(Params {
                player_count: 4,
                player_limit: 8,
                deathmatch: Some(3),
                ..Default::default()
            }),
            30
        );

        assert_eq!(
            calculate_score(Params {
                player_count: 8,
                player_limit: 8,
                deathmatch: Some(3),
                ..Default::default()
            }),
            50
        );

        assert_eq!(
            calculate_score(Params {
                player_count: 8,
                player_limit: 8,
                deathmatch: Some(3),
                spectator_count: 2,
            }),
            60
        );
    }
}
