enum Chessman {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

enum Side {
    White,
    Black,
}

const BOARD_SIZE: usize = 8;

pub struct State {
    pub board: [[Option<(Chessman, Side)>; BOARD_SIZE]; BOARD_SIZE],
}

impl State {
    pub fn new() -> Self {
        Self {
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
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
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
}
