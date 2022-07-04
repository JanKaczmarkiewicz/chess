use std::vec;

use super::{
    chessman::{chessman::Chessman, chessman::ChessmanKind},
    initial_board::INITIAL_BOARD,
    state::Side,
};

pub const BOARD_SIZE: usize = 8;

pub type Tiles = [[Option<Chessman>; BOARD_SIZE]; BOARD_SIZE];

pub struct MoveEntry {
    pub chessman: Chessman,
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub capture: Option<Chessman>,
}

pub type History = Vec<MoveEntry>;

pub struct Board {
    pub tiles: Tiles,
    pub history: History,
}

impl Board {
    pub fn new() -> Self {
        Self {
            history: vec![],
            tiles: INITIAL_BOARD,
        }
    }

    pub fn make_move(&mut self, from_coordinate: (usize, usize), to_coordinate: (usize, usize)) {
        if let Some(from_chessman) = self.tiles[from_coordinate.1][from_coordinate.0].take() {
            let from = from_chessman.clone();
            let to_tile = self.tiles[to_coordinate.1][to_coordinate.0].take();

            if from_chessman.kind == ChessmanKind::King {
                let distance = from_coordinate.0 as i32 - to_coordinate.0 as i32;

                let is_castle_move = distance.abs() == 2;

                if is_castle_move {
                    let is_queen_side_castle = distance < 0;

                    let rook_from_coordinate = if is_queen_side_castle {
                        (7, from_coordinate.1)
                    } else {
                        (0, from_coordinate.1)
                    };

                    let rook_to_coordinate = if is_queen_side_castle {
                        (from_coordinate.0 + 1, from_coordinate.1)
                    } else {
                        (from_coordinate.0 - 1, from_coordinate.1)
                    };
                    let rook = self.tiles[rook_from_coordinate.1][rook_from_coordinate.0].take();
                    self.tiles[rook_to_coordinate.1][rook_to_coordinate.0] = rook;
                }
            }

            self.tiles[to_coordinate.1][to_coordinate.0] = Some(from_chessman);

            // TODO: save castle move
            self.history.push(MoveEntry {
                chessman: from,
                from: from_coordinate,
                to: to_coordinate,
                capture: to_tile,
            });
        }
    }

    pub fn is_coordinate_in_board((x, y): (i32, i32)) -> bool {
        x >= 0 && x < BOARD_SIZE as i32 && y >= 0 && y < BOARD_SIZE as i32
    }

    pub fn is_check_at(tiles: &Tiles, side: &Side, at: (usize, usize)) -> bool {
        for (y, row) in tiles.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(chessman) = cell {
                    let is_ally = &chessman.side == side;

                    if is_ally {
                        continue;
                    }

                    // TODO: history
                    let is_check =
                        Chessman::get_possible_moves(&tiles, (x as i32, y as i32), &vec![])
                            .iter()
                            .any(|possible_move| possible_move.coordinate == at);

                    if !is_check {
                        continue;
                    }

                    return true;
                }
            }
        }

        return false;
    }
}
