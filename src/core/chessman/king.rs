use crate::core::board::Tiles;
use crate::core::state::{PossibleMove, PossibleMoveKind};

use super::super::board::Board;
use super::utils::get_tile;

pub struct King {}

impl King {
    // TODO: implement conditional check logic
    pub fn get_possible_moves(tiles: &Tiles, (x, y): (i32, i32)) -> Vec<PossibleMove> {
        if let Some(chessman) = get_tile(tiles, (x, y)) {
            let mut possible_moves = vec![];

            let directions = &[
                (1, 0),
                (0, 1),
                (0, -1),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ];

            for (x_mod, y_mod) in directions {
                let coordinate = (x + x_mod, y + y_mod);

                if !Board::is_coordinate_in_board(coordinate) {
                    continue;
                }

                if let Some(current_chessman) = get_tile(tiles, coordinate) {
                    if chessman.side != current_chessman.side {
                        possible_moves.push(PossibleMove {
                            kind: PossibleMoveKind::Capture,
                            coordinate: (coordinate.0 as usize, coordinate.1 as usize),
                        });
                    }
                } else {
                    possible_moves.push(PossibleMove {
                        kind: PossibleMoveKind::Move,
                        coordinate: (coordinate.0 as usize, coordinate.1 as usize),
                    });
                }
            }
            return possible_moves;
        }

        vec![]
    }
}
