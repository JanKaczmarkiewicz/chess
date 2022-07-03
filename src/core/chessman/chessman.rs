use super::super::state::PossibleMove;
use super::super::state::Side;
use super::bishop::Bishop;
use super::king::King;
use super::knight::Knight;
use super::pawn::Pawn;
use super::queen::Queen;
use super::rook::Rook;
use super::utils::get_tile;
use crate::core::board::History;
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
    pub fn get_no_check_possible_moves(
        tiles: &Tiles,
        position: (i32, i32),
        history: &History,
    ) -> Vec<PossibleMove> {
        if let Some(chessman) = get_tile(tiles, position) {
            let get_possible_moves = match chessman.kind {
                ChessmanKind::Rook => Rook::get_possible_moves,
                ChessmanKind::Knight => Knight::get_possible_moves,
                ChessmanKind::Bishop => Bishop::get_possible_moves,
                ChessmanKind::Queen => Queen::get_possible_moves,
                ChessmanKind::King => King::get_possible_moves_with_castle,
                ChessmanKind::Pawn => Pawn::get_possible_moves,
            };

            return filter_check_moves(
                &tiles,
                position,
                get_possible_moves(&tiles, position, history),
            );
        }

        vec![]
    }

    pub fn get_possible_moves(
        tiles: &Tiles,
        position: (i32, i32),
        history: &History,
    ) -> Vec<PossibleMove> {
        if let Some(chessman) = get_tile(tiles, position) {
            let get_possible_moves = match chessman.kind {
                ChessmanKind::Rook => Rook::get_possible_moves,
                ChessmanKind::Knight => Knight::get_possible_moves,
                ChessmanKind::Bishop => Bishop::get_possible_moves,
                ChessmanKind::Queen => Queen::get_possible_moves,
                ChessmanKind::King => King::get_possible_moves,
                ChessmanKind::Pawn => Pawn::get_possible_moves,
            };

            return get_possible_moves(tiles, position, history);
        } else {
            vec![]
        }
    }
}
