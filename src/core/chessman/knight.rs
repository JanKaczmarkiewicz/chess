use crate::core::state::{PossibleMove, PossibleMoveKind};

use super::super::board::Board;
use super::super::state::Side;
use super::chessman::{Chessman, ChessmanKind};

pub struct Knight {
    pub side: Side,
}

impl Chessman for Knight {
    fn handle_move(&mut self) {}

    fn get_side(&self) -> &Side {
        &self.side
    }

    fn get_kind(&self) -> ChessmanKind {
        ChessmanKind::Knight
    }

    fn get_possible_moves(&self, board: &Board, (x, y): (i32, i32)) -> Vec<PossibleMove> {
        if board.get_tile((x, y)).is_some() {
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

                if let Some(current_chessman) = board.get_tile(pos) {
                    if self.get_side() != current_chessman.get_side() {
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
