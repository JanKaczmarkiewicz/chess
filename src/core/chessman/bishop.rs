use super::utils::get_direction_possible_movies;
use crate::core::{
    board::{History, Tiles},
    state::PossibleMove,
};

pub struct Bishop {}

impl Bishop {
    pub fn get_possible_moves(
        tiles: &Tiles,
        position: (i32, i32),
        history: &History,
    ) -> Vec<PossibleMove> {
        get_direction_possible_movies(
            tiles,
            position,
            history,
            &[(1, 1), (1, -1), (-1, 1), (-1, -1)],
        )
    }
}
