use super::{
    chessman::{chessman::Chessman, chessman::ChessmanKind},
    state::Side,
};

pub const BOARD_SIZE: usize = 8;

pub type Tiles = [[Option<Chessman>; BOARD_SIZE]; BOARD_SIZE];

pub struct Board {
    pub tiles: Tiles,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [
                [
                    Some(Chessman {
                        kind: ChessmanKind::Rook,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Knight,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Bishop,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Queen,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::King,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Bishop,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Knight,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Rook,
                        side: Side::Black,
                    }),
                ],
                [
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::Black,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::Black,
                    }),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Pawn,
                        side: Side::White,
                    }),
                ],
                [
                    Some(Chessman {
                        kind: ChessmanKind::Rook,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Knight,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Bishop,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Queen,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::King,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Bishop,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Knight,
                        side: Side::White,
                    }),
                    Some(Chessman {
                        kind: ChessmanKind::Rook,
                        side: Side::White,
                    }),
                ],
            ],
        }
    }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        let from_chessman = self.tiles[from.1][from.0].take();
        self.tiles[to.1][to.0] = from_chessman;
    }

    pub fn is_coordinate_in_board((x, y): (i32, i32)) -> bool {
        x >= 0 && x < BOARD_SIZE as i32 && y >= 0 && y < BOARD_SIZE as i32
    }

    pub fn is_check(tiles: &Tiles, side: &Side) -> bool {
        let mut enemy_possible_moves = tiles.iter().enumerate().flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter_map({
                    let side = side.clone();
                    move |(x, tile)| {
                        tile.as_ref().map(|chessman| {
                            let is_enemy = chessman.side != side;

                            if is_enemy {
                                Chessman::get_possible_moves(&tiles, (x as i32, y as i32))
                                    .into_iter()
                                    .map(|possible_move| possible_move.coordinate)
                                    .collect()
                            } else {
                                vec![]
                            }
                        })
                    }
                })
                .flatten()
        });

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

        enemy_possible_moves.any(|possible_move| possible_move == king_position)
    }
}
