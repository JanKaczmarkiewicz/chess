use super::super::state::PossibleMove;
use super::super::state::PossibleMoveKind;
use super::chessman::{Chessman, ChessmanKind};
use crate::core::state::State;
use crate::core::state::{History, Tiles, BOARD_SIZE};
use std::vec;

pub fn get_direction_possible_movies(
    tiles: &Tiles,
    (x, y): (i32, i32),
    _history: &History,
    directions: &[(i32, i32)],
) -> Vec<PossibleMove> {
    if let Some(chessman) = get_tile(tiles, (x, y)) {
        let mut possible_moves = vec![];

        for (x_mod, y_mod) in directions {
            for i in 1..BOARD_SIZE {
                let x_next = x + x_mod * i as i32;
                let y_next = y + y_mod * i as i32;

                let coordinate = (x_next, y_next);

                if !State::is_coordinate_in_board(coordinate) {
                    break;
                }

                if let Some(current_chessman) = get_tile(tiles, coordinate) {
                    if chessman.side != current_chessman.side {
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

    vec![]
}

pub fn filter_check_moves(
    tiles: &Tiles,
    (x, y): (i32, i32),
    unfiltered_possible_moves: Vec<PossibleMove>,
) -> Vec<PossibleMove> {
    unfiltered_possible_moves
        .into_iter()
        .filter(|possible_move| {
            let (possible_move_x, possible_move_y) = possible_move.coordinate;
            let mut tiles = tiles.clone();

            // inlined move
            let from_chessman = tiles[y as usize][x as usize].take();
            tiles[possible_move_y][possible_move_x] = from_chessman;

            let side = &tiles[possible_move_y][possible_move_x]
                .clone()
                .unwrap()
                .side;

            let king_position = tiles
                .iter()
                .enumerate()
                .find_map(|(y, row)| {
                    row.into_iter().enumerate().find_map({
                        |(x, tile)| {
                            tile.as_ref().and_then(|chessman| {
                                if &chessman.side == side && chessman.kind == ChessmanKind::King {
                                    Some((x as usize, y as usize))
                                } else {
                                    None
                                }
                            })
                        }
                    })
                })
                .expect("There should be always a king on the board");

            let is_check = !State::is_check_at(&tiles, side, king_position);

            is_check
        })
        .collect()
}

pub fn get_tile(tiles: &Tiles, (x, y): (i32, i32)) -> Option<&Chessman> {
    if State::is_coordinate_in_board((x, y)) {
        tiles[y as usize][x as usize].as_ref()
    } else {
        None
    }
}
