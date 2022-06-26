use std::vec;

use super::super::state::PossibleMoveKind;
use super::super::{board::Board, state::PossibleMove};

pub fn get_direction_possible_movies(
    board: &Board,
    (x, y): (i32, i32),
    directions: &[(i32, i32)],
) -> Vec<PossibleMove> {
    if let Some(chessman) = board.get_tile((x, y)) {
        let mut possible_moves = vec![];

        for (x_mod, y_mod) in directions {
            for i in 1..board.get_board_size() {
                let x_next = x + x_mod * i as i32;
                let y_next = y + y_mod * i as i32;

                let coordinate = (x_next, y_next);

                if !Board::is_coordinate_in_board(coordinate) {
                    break;
                }

                if let Some(current_chessman) = board.get_tile(coordinate) {
                    if chessman.get_side() != current_chessman.get_side() {
                        possible_moves.push(PossibleMove {
                            kind: PossibleMoveKind::Capture,
                            coordinate: (coordinate.0 as usize, coordinate.1 as usize),
                        });
                    }

                    break;
                } else {
                    possible_moves.push(PossibleMove {
                        kind: PossibleMoveKind::Move,
                        coordinate: (coordinate.0 as usize, coordinate.1 as usize),
                    });
                }
            }
        }

        return possible_moves;
    }

    return vec![];
}
