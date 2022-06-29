use super::super::board::Board;
use super::utils::get_tile;
use crate::core::board::Tiles;
use crate::core::state::{PossibleMove, PossibleMoveKind};

pub struct Knight {}

impl Knight {
    pub fn get_possible_moves(tiles: &Tiles, (x, y): (i32, i32)) -> Vec<PossibleMove> {
        if let Some(chessman) = get_tile(tiles, (x, y)) {
            let directions = [
                (-1, 2),
                (-1, -2),
                (1, 2),
                (1, -2),
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
            ];

            let mut possible_moves = vec![];

            for (x_mod, y_mod) in directions {
                let pos = (x + x_mod, y + y_mod);

                if !Board::is_coordinate_in_board(pos) {
                    continue;
                }

                if let Some(current_chessman) = get_tile(tiles, pos) {
                    if chessman.side != current_chessman.side {
                        possible_moves.push(PossibleMove {
                            kind: PossibleMoveKind::Capture,
                            coordinate: (pos.0 as usize, pos.1 as usize),
                        });
                    }
                } else {
                    possible_moves.push(PossibleMove {
                        kind: PossibleMoveKind::Move,
                        coordinate: (pos.0 as usize, pos.1 as usize),
                    });
                }
            }

            return possible_moves;
        }

        vec![]
    }
}
