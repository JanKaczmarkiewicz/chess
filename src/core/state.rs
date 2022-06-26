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
            current_side: Side::White,
            selected_tile: None,
            possible_moves: vec![],
            board: Board::new(),
        }
    }

    pub fn get_board(&self) -> &Tiles {
        &self.board.tiles
    }

    pub fn handle_action(&mut self, input: (i32, i32)) {
        self.possible_moves = vec![];

        if let Some(chessman) = self.board.get_tile(input) {
            if chessman.get_side() == &self.current_side {
                self.possible_moves = chessman.get_possible_moves(&self.board, input);
                self.selected_tile = Some((input.0 as usize, input.1 as usize));

                return;
            }
        };
        self.selected_tile = None;
    }
}
