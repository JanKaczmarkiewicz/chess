use super::chessman::chessman::{Chessman, ChessmanKind};
use super::chessman::utils::get_tile;
use super::initial_board::INITIAL_BOARD;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Side {
    White,
    Black,
}

#[derive(Eq, PartialEq, Debug)]
pub enum PossibleMoveKind {
    Move,
    Capture,
}

pub const BOARD_SIZE: usize = 8;

pub type Tiles = [[Option<Chessman>; BOARD_SIZE]; BOARD_SIZE];

pub struct MoveEntry {
    pub chessman: Chessman,
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub capture: Option<Chessman>,
}

pub type History = Vec<MoveEntry>;

#[derive(Eq, PartialEq, Debug)]
pub struct PossibleMove {
    pub kind: PossibleMoveKind,
    pub coordinate: (usize, usize),
}

pub struct State {
    pub selected_tile: Option<(usize, usize)>,
    pub current_side: Side,
    pub possible_moves: Vec<PossibleMove>,
    pub tiles: Tiles,
    pub history: History,
}

impl Default for State {
    fn default() -> Self {
        Self::new(INITIAL_BOARD)
    }
}

impl State {
    pub fn new(tiles: Tiles) -> Self {
        Self {
            current_side: Side::White,
            history: vec![],
            possible_moves: vec![],
            selected_tile: None,
            tiles,
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
                        Chessman::get_possible_moves(tiles, (x as i32, y as i32), &vec![])
                            .iter()
                            .any(|possible_move| possible_move.coordinate == at);

                    if !is_check {
                        continue;
                    }

                    return true;
                }
            }
        }

        false
    }

    fn make_move(&mut self, from_coordinate: (usize, usize), to_coordinate: (usize, usize)) {
        if let Some(from_chessman) = self.tiles[from_coordinate.1][from_coordinate.0].take() {
            let from = from_chessman.clone();
            let to_tile = self.tiles[to_coordinate.1][to_coordinate.0].take();

            if from_chessman.kind == ChessmanKind::King {
                let distance = from_coordinate.0 as i32 - to_coordinate.0 as i32;

                let is_castle_move = distance.abs() == 2;

                if is_castle_move {
                    let is_queen_side_castle = distance < 0;

                    let rook_from_coordinate = if is_queen_side_castle {
                        (BOARD_SIZE - 1, from_coordinate.1)
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

    pub fn get_board(&self) -> &Tiles {
        &self.tiles
    }

    pub fn handle_action(&mut self, input: (i32, i32)) {
        let coordinate = (input.0 as usize, input.1 as usize);

        if let Some(selected_tile) = self.selected_tile {
            let is_possible_move = self
                .possible_moves
                .iter()
                .any(|x| coordinate == x.coordinate);

            if is_possible_move {
                self.make_move(selected_tile, coordinate);
                self.current_side = match self.current_side {
                    Side::White => Side::Black,
                    Side::Black => Side::White,
                };
                self.selected_tile = None;
                self.possible_moves = vec![];

                return;
            }

            if let Some(selected_chessman) = get_tile(
                &self.tiles,
                (selected_tile.0 as i32, selected_tile.1 as i32),
            ) {
                if let Some(clicked_chessman) = get_tile(&self.tiles, input) {
                    if selected_chessman.side == clicked_chessman.side {
                        self.possible_moves = Chessman::get_no_check_possible_moves(
                            &self.tiles,
                            input,
                            &self.history,
                        );
                        self.selected_tile = Some(coordinate);

                        return;
                    }
                }
            }

            self.selected_tile = None;
            self.possible_moves = vec![];

            return;
        }

        if let Some(chessman) = get_tile(&self.tiles, input) {
            if chessman.side == self.current_side {
                self.possible_moves =
                    Chessman::get_no_check_possible_moves(&self.tiles, input, &self.history);
                self.selected_tile = Some(coordinate);

                return;
            }
        };

        self.selected_tile = None;
        self.possible_moves = vec![];
    }
}
