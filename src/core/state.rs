use super::board::{Board, Tiles};

#[derive(Eq, PartialEq, Clone)]
pub enum Side {
    White,
    Black,
}

#[derive(Eq, PartialEq, Debug)]
pub enum PossibleMoveKind {
    Move,
    Capture,
}

#[derive(Eq, PartialEq, Debug)]
pub struct PossibleMove {
    pub kind: PossibleMoveKind,
    pub coordinate: (usize, usize),
}

pub struct State {
    pub board: Board,
    pub selected_tile: Option<(usize, usize)>,
    pub current_side: Side,
    pub possible_moves: Vec<PossibleMove>,
}

impl State {
    pub fn new() -> Self {
        Self {
            current_side: Side::Black,
            selected_tile: None,
            possible_moves: vec![],
            board: Board::new(),
        }
    }

    pub fn get_board(&self) -> &Tiles {
        &self.board.tiles
    }

    pub fn handle_action(&mut self, input: (i32, i32)) {
        let coordinate = (input.0 as usize, input.1 as usize);

        if let Some(selected_tile) = self.selected_tile {
            let is_possible_move = self
                .possible_moves
                .iter()
                .find(|x| coordinate == x.coordinate)
                .is_some();

            if is_possible_move {
                self.board.make_move(selected_tile, coordinate);
                self.current_side = match self.current_side {
                    Side::White => Side::Black,
                    Side::Black => Side::White,
                };
                self.selected_tile = None;
                self.possible_moves = vec![];

                return;
            }

            if let Some(selected_chessman) = self
                .board
                .get_tile((selected_tile.0 as i32, selected_tile.1 as i32))
            {
                if let Some(clicked_chessman) = self.board.get_tile(input) {
                    if selected_chessman.get_side() == clicked_chessman.get_side() {
                        self.possible_moves =
                            clicked_chessman.get_possible_moves(&self.board, input);
                        self.selected_tile = Some(coordinate);

                        return;
                    }
                }
            }

            self.selected_tile = None;
            self.possible_moves = vec![];

            return;
        }

        if let Some(chessman) = self.board.get_tile(input) {
            if chessman.get_side() == &self.current_side {
                self.possible_moves = chessman.get_possible_moves(&self.board, input);
                self.selected_tile = Some(coordinate);

                return;
            }
        };

        self.selected_tile = None;
        self.possible_moves = vec![];
    }
}
