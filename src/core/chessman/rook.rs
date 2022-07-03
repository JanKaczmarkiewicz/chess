use crate::core::board::{History, Tiles};
use crate::core::state::PossibleMove;

use super::utils::get_direction_possible_movies;

pub struct Rook {}

impl Rook {
    pub fn get_possible_moves(
        tiles: &Tiles,
        position: (i32, i32),
        history: &History,
    ) -> Vec<PossibleMove> {
        get_direction_possible_movies(
            tiles,
            position,
            history,
            &[(1, 0), (0, 1), (0, -1), (-1, 0)],
        )
    }
}
