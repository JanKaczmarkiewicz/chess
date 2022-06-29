use crate::core::board::{Tiles, BOARD_SIZE};
use crate::core::state::{PossibleMove, PossibleMoveKind};

use super::super::state::Side;
use super::utils::get_tile;

pub struct Pawn {}

impl Pawn {
    pub fn get_possible_moves(tiles: &Tiles, (x, y): (i32, i32)) -> Vec<PossibleMove> {
        if let Some(chessman) = get_tile(tiles, (x, y)) {
            let direction = match chessman.side {
                Side::White => -1,
                Side::Black => 1,
            };

            let mut possible_moves = vec![];

            // move
            let starting_position = match chessman.side {
                Side::White => BOARD_SIZE - 2,
                Side::Black => 1,
            };

            let len = if starting_position == y as usize {
                2
            } else {
                1
            };

            for step in 1..=len {
                let coordinate = (x, y + step * direction);

                if get_tile(tiles, coordinate).is_none() {
                    possible_moves.push(PossibleMove {
                        kind: PossibleMoveKind::Move,
                        coordinate: (coordinate.0 as usize, coordinate.1 as usize),
                    });
                };
            }

            // capture
            for right_or_left in [-1, 1] {
                let coordinate = (x + right_or_left, y + direction);

                if let Some(current_chessman) = get_tile(tiles, coordinate) {
                    if chessman.side != current_chessman.side {
                        possible_moves.push(PossibleMove {
                            kind: PossibleMoveKind::Capture,
                            coordinate: (coordinate.0 as usize, coordinate.1 as usize),
                        });
                    }
                }
            }

            return possible_moves;
        }

        vec![]
    }
}
