use crate::core::state::{PossibleMove, PossibleMoveKind};

use super::super::board::Board;
use super::super::state::Side;
use super::chessman::{Chessman, ChessmanKind};
use super::utils::filter_check_moves;

pub struct Pawn {
    pub side: Side,
}

impl Chessman for Pawn {
    fn handle_move(&mut self) {}

    fn get_side(&self) -> &Side {
        &self.side
    }

    fn get_kind(&self) -> ChessmanKind {
        ChessmanKind::Pawn
    }

    fn get_possible_moves(&self, board: &Board, (x, y): (i32, i32)) -> Vec<PossibleMove> {
        if board.get_tile((x, y)).is_some() {
            let direction = match self.side {
                Side::White => -1,
                Side::Black => 1,
            };

            let mut possible_moves = vec![];

            // move
            let starting_position = match self.side {
                Side::White => board.get_board_size() - 2,
                Side::Black => 1,
            };

            let len = if starting_position == y as usize {
                2
            } else {
                1
            };

            for step in 1..=len {
                let coordinate = (x, y + step * direction);

                if board.get_tile(coordinate).is_none() {
                    possible_moves.push(PossibleMove {
                        kind: PossibleMoveKind::Move,
                        coordinate: (coordinate.0 as usize, coordinate.1 as usize),
                    });
                };
            }

            // capture
            let right_and_left = [-1, 1];

            for right_or_left in right_and_left {
                let coordinate = (x + right_or_left, y + direction);

                if let Some(chessman) = board.get_tile(coordinate) {
                    if chessman.get_side() != &self.side {
                        possible_moves.push(PossibleMove {
                            kind: PossibleMoveKind::Capture,
                            coordinate: (coordinate.0 as usize, coordinate.1 as usize),
                        });
                    }
                }
            }

            return filter_check_moves(board, (x, y), possible_moves);
        }

        vec![]
    }
}

impl Pawn {
    pub fn new(side: Side) -> Self {
        Pawn { side }
    }
}
