use crate::core::state::{PossibleMove, PossibleMoveKind};

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

    // TODO: implement check logic
    fn get_possible_moves(&self, board: &Board, (x, y): (i32, i32)) -> Vec<PossibleMove> {
        if board.get_tile((x, y)).is_none() {
            return vec![];
        }

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

            if let Some(current_chessman) = board.get_tile(coordinate) {
                if self.get_side() != current_chessman.get_side() {
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
}
