use crate::GameServer;

mod weight {
    pub const FULL_SERVER: f32 = 20.;
    pub const HUMAN_PLAYER: f32 = 4.;
    pub const SPECTATOR: f32 = 2.;
    pub const DMM4_FACTOR: f32 = 0.5;
}

pub(crate) fn from_game_server(server: &GameServer) -> u32 {
    let human_count = server.players().iter().filter(|p| !p.is_bot()).count() as f32;

    if 1. == human_count {
        return weight::HUMAN_PLAYER as u32;
    }

    let server_score = {
        let fill_percentage = {
            let expected_count = server.settings().maxclients.map_or(8., |max| max as f32);
            (human_count / expected_count).min(1.0)
        };
        let score_factor = match server.settings().deathmatch.is_none_or(|dm| 4 == dm) {
            true => weight::DMM4_FACTOR,
            false => 1.0,
        };
        fill_percentage * weight::FULL_SERVER * score_factor
    };

    let player_score = human_count * weight::HUMAN_PLAYER;

    let spectator_score = {
        let spectator_count = {
            let qtv_spectator_count = server.qtv_stream().map_or(0, |q| q.client_count());
            server.spectators().len() as u32 + qtv_spectator_count
        } as f32;

        spectator_count * weight::SPECTATOR
    };

    let score_sum = server_score + player_score + spectator_score;
    let floored_sum = (score_sum / 10.).floor() * 10.;
    floored_sum as u32
}

// todo: tests, coverage
/*
 #[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_game_server() {
    }
}
*/
