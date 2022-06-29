use super::super::state::PossibleMove;
use super::super::state::Side;
use super::bishop::Bishop;
use super::king::King;
use super::knight::Knight;
use super::pawn::Pawn;
use super::queen::Queen;
use super::rook::Rook;
use super::utils::get_tile;
use crate::core::board::Tiles;

use super::utils::filter_check_moves;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChessmanKind {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Clone, Debug)]
pub struct Chessman {
    pub kind: ChessmanKind,
    pub side: Side,
}

impl Chessman {
    pub fn get_filtered_possible_moves(tiles: &Tiles, position: (i32, i32)) -> Vec<PossibleMove> {
        return filter_check_moves(&tiles, position, Self::get_possible_moves(&tiles, position));
    }

    pub fn get_possible_moves(tiles: &Tiles, position: (i32, i32)) -> Vec<PossibleMove> {
        if let Some(chessman) = get_tile(tiles, position) {
            match chessman.kind {
                ChessmanKind::Rook => Rook::get_possible_moves(tiles, position),
                ChessmanKind::Knight => Knight::get_possible_moves(tiles, position),
                ChessmanKind::Bishop => Bishop::get_possible_moves(tiles, position),
                ChessmanKind::Queen => Queen::get_possible_moves(tiles, position),
                ChessmanKind::King => King::get_possible_moves(tiles, position),
                ChessmanKind::Pawn => Pawn::get_possible_moves(tiles, position),
            }
        } else {
            vec![]
        }
    }
}
