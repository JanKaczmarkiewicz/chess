use super::utils::get_direction_possible_movies;
use crate::core::state::{History, PossibleMove, Tiles};

pub struct Queen {}

impl Queen {
    pub fn get_possible_moves(
        tiles: &Tiles,
        position: (i32, i32),
        history: &History,
    ) -> Vec<PossibleMove> {
        get_direction_possible_movies(
            tiles,
            position,
            history,
            &[
                (1, 0),
                (0, 1),
                (0, -1),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ],
        )
    }
}
