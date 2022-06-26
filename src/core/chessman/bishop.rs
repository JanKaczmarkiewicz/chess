use crate::core::state::PossibleMove;

use super::super::board::Board;
use super::super::state::Side;
use super::chessman::{Chessman, ChessmanKind};
use super::utils::get_direction_possible_movies;

pub struct Bishop {
    pub side: Side,
}

impl Chessman for Bishop {
    fn get_side(&self) -> &Side {
        return &self.side;
    }

    fn get_kind(&self) -> ChessmanKind {
        return ChessmanKind::Bishop;
    }

    fn get_possible_moves(&self, board: &Board, position: (i32, i32)) -> Vec<PossibleMove> {
        get_direction_possible_movies(board, position, &[(1, 1), (1, -1), (-1, 1), (-1, -1)])
    }
}
