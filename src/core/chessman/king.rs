use crate::core::state::{History, State, Tiles, BOARD_SIZE};
use crate::core::state::{PossibleMove, PossibleMoveKind};

use super::utils::get_tile;

pub struct King {}

impl King {
    pub fn get_possible_moves_with_castle(
        tiles: &Tiles,
        (x, y): (i32, i32),
        history: &History,
    ) -> Vec<PossibleMove> {
        let mut possible_moves = Self::get_possible_moves(tiles, (x, y), history);

        if let Some(chessman) = get_tile(tiles, (x, y)) {
            let is_king_moved = history
                .iter()
                .find(|entry| {
                    entry.chessman.side == chessman.side && entry.chessman.kind == chessman.kind
                })
                .is_some();

            if is_king_moved {
                return possible_moves;
            }

            let king_starting_y_position = y as usize;

            let initial_rooks_positions = [
                (0, king_starting_y_position),
                (BOARD_SIZE - 1, king_starting_y_position),
            ];

            for rook_pos in initial_rooks_positions {
                let is_rook_moved = history
                    .iter()
                    .find(|entry| entry.from == rook_pos || entry.to == rook_pos)
                    .is_some();

                if is_rook_moved {
                    continue;
                }

                let king_starting_x_position = x as usize;

                let is_queen_side_castle = rook_pos.0 < king_starting_x_position;

                let mut x_range = if is_queen_side_castle {
                    rook_pos.0 + 1..king_starting_x_position
                } else {
                    (king_starting_x_position + 1)..rook_pos.0
                };

                let is_chessman_between_king_and_rook =
                    x_range.any(|x| get_tile(tiles, (x as i32, y as i32)).is_some());

                if is_chessman_between_king_and_rook {
                    continue;
                }

                let coordinates_expected_no_check = if is_queen_side_castle {
                    [
                        (king_starting_x_position - 2, king_starting_y_position),
                        (king_starting_x_position - 1, king_starting_y_position),
                        (king_starting_x_position, king_starting_y_position),
                    ]
                } else {
                    [
                        (king_starting_x_position, king_starting_y_position),
                        (king_starting_x_position + 1, king_starting_y_position),
                        (king_starting_x_position + 2, king_starting_y_position),
                    ]
                };

                let is_check_at_path = coordinates_expected_no_check
                    .iter()
                    .any(|coordinate| State::is_check_at(tiles, &chessman.side, *coordinate));

                if is_check_at_path {
                    continue;
                }

                possible_moves.push(PossibleMove {
                    kind: PossibleMoveKind::Move,
                    coordinate: (
                        if is_queen_side_castle {
                            king_starting_x_position - 2
                        } else {
                            king_starting_x_position + 2
                        },
                        king_starting_y_position,
                    ),
                });
            }
        }

        return possible_moves;
    }

    pub fn get_possible_moves(
        tiles: &Tiles,
        (x, y): (i32, i32),
        _history: &History,
    ) -> Vec<PossibleMove> {
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

                if !State::is_coordinate_in_board(coordinate) {
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
