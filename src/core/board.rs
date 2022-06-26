use super::{
    chessman::{
        bishop::Bishop, chessman::Chessman, king::King, knight::Knight, pawn::Pawn, queen::Queen,
        rook::Rook,
    },
    state::Side,
};

const BOARD_SIZE: usize = 8;

pub type Tiles = [[Option<Box<dyn Chessman>>; BOARD_SIZE]; BOARD_SIZE];

pub struct Board {
    pub tiles: Tiles,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [
                [
                    Some(Box::new(Rook { side: Side::Black })),
                    Some(Box::new(Knight { side: Side::Black })),
                    Some(Box::new(Bishop { side: Side::Black })),
                    Some(Box::new(Queen { side: Side::Black })),
                    Some(Box::new(King { side: Side::Black })),
                    Some(Box::new(Bishop { side: Side::Black })),
                    Some(Box::new(Knight { side: Side::Black })),
                    Some(Box::new(Rook { side: Side::Black })),
                ],
                [
                    Some(Box::new(Pawn::new(Side::Black))),
                    Some(Box::new(Pawn::new(Side::Black))),
                    Some(Box::new(Pawn::new(Side::Black))),
                    Some(Box::new(Pawn::new(Side::Black))),
                    Some(Box::new(Pawn::new(Side::Black))),
                    Some(Box::new(Pawn::new(Side::Black))),
                    Some(Box::new(Pawn::new(Side::Black))),
                    Some(Box::new(Pawn::new(Side::Black))),
                ],
                [
                    None,
                    None,
                    None,
                    None,
                    Some(Box::new(Pawn::new(Side::White))),
                    None,
                    None,
                    None,
                ],
                [
                    None,
                    Some(Box::new(Pawn::new(Side::White))),
                    Some(Box::new(Rook { side: Side::White })),
                    None,
                    Some(Box::new(Bishop { side: Side::White })),
                    None,
                    None,
                    None,
                ],
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Box::new(Queen { side: Side::White })),
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Box::new(Pawn::new(Side::White))),
                    Some(Box::new(Pawn::new(Side::White))),
                    Some(Box::new(Pawn::new(Side::White))),
                    Some(Box::new(Pawn::new(Side::White))),
                    Some(Box::new(Pawn::new(Side::White))),
                    Some(Box::new(Pawn::new(Side::White))),
                    Some(Box::new(Pawn::new(Side::White))),
                    Some(Box::new(Pawn::new(Side::White))),
                ],
                [
                    Some(Box::new(Rook { side: Side::White })),
                    Some(Box::new(Knight { side: Side::White })),
                    Some(Box::new(Bishop { side: Side::White })),
                    Some(Box::new(Queen { side: Side::White })),
                    Some(Box::new(King { side: Side::White })),
                    Some(Box::new(Bishop { side: Side::White })),
                    Some(Box::new(Knight { side: Side::White })),
                    Some(Box::new(Rook { side: Side::White })),
                ],
            ],
        }
    }

    pub fn get_board_size(&self) -> usize {
        self.tiles.len()
    }

    pub fn is_coordinate_in_board((x, y): (i32, i32)) -> bool {
        x >= 0 && x < BOARD_SIZE as i32 && y >= 0 && y < BOARD_SIZE as i32
    }

    pub fn get_tile(&self, (x, y): (i32, i32)) -> &Option<Box<dyn Chessman>> {
        if Self::is_coordinate_in_board((x, y)) {
            &self.tiles[y as usize][x as usize]
        } else {
            &None
        }
    }
}
