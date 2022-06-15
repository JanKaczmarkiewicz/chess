pub enum Chessman {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Eq, PartialEq)]
pub enum Side {
    White,
    Black,
}

const BOARD_SIZE: usize = 8;

pub struct State {
    pub board: [[Option<(Chessman, Side)>; BOARD_SIZE]; BOARD_SIZE],
    pub selected_tile: Option<(usize, usize)>,
    pub current_side: Side,
    pub possible_moves: Vec<(usize, usize)>,
}

impl State {
    pub fn new() -> Self {
        Self {
            current_side: Side::White,
            selected_tile: None,
            possible_moves: vec![],
            board: [
                [
                    Some((Chessman::Rook, Side::Black)),
                    Some((Chessman::Knight, Side::Black)),
                    Some((Chessman::Bishop, Side::Black)),
                    Some((Chessman::Queen, Side::Black)),
                    Some((Chessman::King, Side::Black)),
                    Some((Chessman::Bishop, Side::Black)),
                    Some((Chessman::Knight, Side::Black)),
                    Some((Chessman::Rook, Side::Black)),
                ],
                [
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                ],
                [
                    Some((Chessman::Rook, Side::White)),
                    Some((Chessman::Knight, Side::White)),
                    Some((Chessman::Bishop, Side::White)),
                    Some((Chessman::Queen, Side::White)),
                    Some((Chessman::King, Side::White)),
                    Some((Chessman::Bishop, Side::White)),
                    Some((Chessman::Knight, Side::White)),
                    Some((Chessman::Rook, Side::White)),
                ],
            ],
        }
    }

    pub fn handle_action(&mut self, (x, y): (usize, usize)) {
        if let Some((chessman, side)) = &self.board[y][x] {
            if side == &self.current_side {
                self.selected_tile = Some((x, y));
            }
        };
    }
}
