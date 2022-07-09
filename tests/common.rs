use chess::core::{
    chessman::chessman::{Chessman, ChessmanKind},
    state::{Side, State, Tiles},
};

pub fn from_literal(literal: &str) -> State {
    let mut tiles: Tiles = Default::default();

    literal
        .trim()
        .lines()
        .skip(1)
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .skip(1)
                .enumerate()
                .filter(|(x, _c)| x % 2 == 1)
                .map(|(_x, c)| c)
                .enumerate()
                .for_each(|(x, c)| {
                    tiles[y][x] = match c {
                        '♜' => Some(Chessman {
                            kind: ChessmanKind::Rook,
                            side: Side::Black,
                        }),
                        '♞' => Some(Chessman {
                            kind: ChessmanKind::Knight,
                            side: Side::Black,
                        }),
                        '♝' => Some(Chessman {
                            kind: ChessmanKind::Bishop,
                            side: Side::Black,
                        }),
                        '♛' => Some(Chessman {
                            kind: ChessmanKind::Queen,
                            side: Side::Black,
                        }),
                        '♚' => Some(Chessman {
                            kind: ChessmanKind::King,
                            side: Side::Black,
                        }),
                        '♟' => Some(Chessman {
                            kind: ChessmanKind::Pawn,
                            side: Side::Black,
                        }),
                        '♖' => Some(Chessman {
                            kind: ChessmanKind::Rook,
                            side: Side::White,
                        }),
                        '♘' => Some(Chessman {
                            kind: ChessmanKind::Knight,
                            side: Side::White,
                        }),
                        '♗' => Some(Chessman {
                            kind: ChessmanKind::Bishop,
                            side: Side::White,
                        }),
                        '♕' => Some(Chessman {
                            kind: ChessmanKind::Queen,
                            side: Side::White,
                        }),
                        '♔' => Some(Chessman {
                            kind: ChessmanKind::King,
                            side: Side::White,
                        }),
                        '♙' => Some(Chessman {
                            kind: ChessmanKind::Pawn,
                            side: Side::White,
                        }),
                        _ => None,
                    };
                });
        });

    State::new(tiles)
}

pub fn to_literal(state: &State) -> String {
    let mut literal = String::from("\n  0 1 2 3 4 5 6 7\n");

    let rows = state
        .tiles
        .iter()
        .enumerate()
        .map(|(y, row)| {
            y.to_string()
                + " "
                + &row
                    .iter()
                    .enumerate()
                    .map(|(_x, cell)| match cell {
                        Some(chessman) => match chessman.side {
                            Side::Black => match chessman.kind {
                                ChessmanKind::Rook => "♜",
                                ChessmanKind::Knight => "♞",
                                ChessmanKind::Bishop => "♝",
                                ChessmanKind::Queen => "♛",
                                ChessmanKind::King => "♚",
                                ChessmanKind::Pawn => "♟",
                            },
                            Side::White => match chessman.kind {
                                ChessmanKind::Rook => "♖",
                                ChessmanKind::Knight => "♘",
                                ChessmanKind::Bishop => "♗",
                                ChessmanKind::Queen => "♕",
                                ChessmanKind::King => "♔",
                                ChessmanKind::Pawn => "♙",
                            },
                        },
                        None => ".",
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    literal.push_str(rows.as_str());

    literal
}
