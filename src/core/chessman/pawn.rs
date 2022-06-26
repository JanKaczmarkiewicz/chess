use crate::core::state::PossibleMove;

use super::super::board::Board;
use super::super::state::Side;
use super::chessman::{Chessman, ChessmanKind};

pub struct Pawn {
    pub side: Side,
}

impl Chessman for Pawn {
    fn get_side(&self) -> &Side {
        return &self.side;
    }

    fn get_kind(&self) -> ChessmanKind {
        return ChessmanKind::Pawn;
    }

    fn get_possible_moves(&self, board: &Board, position: (i32, i32)) -> Vec<PossibleMove> {
        return vec![];
    }
}
