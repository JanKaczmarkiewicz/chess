use super::super::state::Side;
use super::super::{board::Board, state::PossibleMove};

pub enum ChessmanKind {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

pub trait Chessman {
    fn handle_move(&mut self) -> ();
    fn get_side(&self) -> &Side;
    fn get_kind(&self) -> ChessmanKind;
    fn get_possible_moves(&self, board: &Board, position: (i32, i32)) -> Vec<PossibleMove>;
}
