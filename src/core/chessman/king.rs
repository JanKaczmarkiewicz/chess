use crate::core::state::PossibleMove;

use super::super::board::Board;
use super::super::state::Side;
use super::chessman::{Chessman, ChessmanKind};

pub struct King {
    pub side: Side,
}

impl Chessman for King {
    fn get_side(&self) -> &Side {
        &self.side
    }

    fn handle_move(&mut self) {}

    fn get_kind(&self) -> ChessmanKind {
        ChessmanKind::King
    }

    fn get_possible_moves(&self, board: &Board, position: (i32, i32)) -> Vec<PossibleMove> {
        return vec![];
    }
}
