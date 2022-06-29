use super::utils::get_direction_possible_movies;
use crate::core::{board::Tiles, state::PossibleMove};

pub struct Bishop {}

impl Bishop {
    pub fn get_possible_moves(tiles: &Tiles, position: (i32, i32)) -> Vec<PossibleMove> {
        get_direction_possible_movies(tiles, position, &[(1, 1), (1, -1), (-1, 1), (-1, -1)])
    }
}
