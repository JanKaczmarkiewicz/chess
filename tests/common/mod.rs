use chess::core::{
    chessman::chessman::{
        Chessman,
        ChessmanKind::{Bishop, King, Knight, Pawn, Queen, Rook},
    },
    state::{
        Side::{Black, White},
        State, Tiles,
    },
};

pub fn from_literal(literal: &str) -> State {
    let mut tiles: Tiles = Default::default();

    literal
        .trim()
        .lines()
        .skip(1)
        .enumerate()
        .for_each(|(y, line)| {
            line.trim()
                .chars()
                .skip(1)
                .enumerate()
                .filter(|(x, _c)| x % 2 == 1)
                .map(|(_x, c)| c)
                .enumerate()
                .for_each(|(x, c)| {
                    tiles[y][x] = match c {
                        '♜' => Some(Chessman {
                            kind: Rook,
                            side: Black,
                        }),
                        '♞' => Some(Chessman {
                            kind: Knight,
                            side: Black,
                        }),
                        '♝' => Some(Chessman {
                            kind: Bishop,
                            side: Black,
                        }),
                        '♛' => Some(Chessman {
                            kind: Queen,
                            side: Black,
                        }),
                        '♚' => Some(Chessman {
                            kind: King,
                            side: Black,
                        }),
                        '♟' => Some(Chessman {
                            kind: Pawn,
                            side: Black,
                        }),
                        '♖' => Some(Chessman {
                            kind: Rook,
                            side: White,
                        }),
                        '♘' => Some(Chessman {
                            kind: Knight,
                            side: White,
                        }),
                        '♗' => Some(Chessman {
                            kind: Bishop,
                            side: White,
                        }),
                        '♕' => Some(Chessman {
                            kind: Queen,
                            side: White,
                        }),
                        '♔' => Some(Chessman {
                            kind: King,
                            side: White,
                        }),
                        '♙' => Some(Chessman {
                            kind: Pawn,
                            side: White,
                        }),
                        _ => None,
                    };
                });
        });

    State::new(tiles)
}

pub fn to_literal(state: &State) -> String {
    let mut literal = String::from("\n          0 1 2 3 4 5 6 7\n");

    let rows = state
        .tiles
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let number = y.to_string();
            let row = &row
                .iter()
                .enumerate()
                .map(|(_x, cell)| match cell {
                    Some(chessman) => match chessman.side {
                        Black => match chessman.kind {
                            Rook => "♜",
                            Knight => "♞",
                            Bishop => "♝",
                            Queen => "♛",
                            King => "♚",
                            Pawn => "♟",
                        },
                        White => match chessman.kind {
                            Rook => "♖",
                            Knight => "♘",
                            Bishop => "♗",
                            Queen => "♕",
                            King => "♔",
                            Pawn => "♙",
                        },
                    },
                    None => ".",
                })
                .collect::<Vec<_>>()
                .join(" ");

            format!("        {number} {row}")
        })
        .collect::<Vec<_>>()
        .join("\n");

    literal.push_str(rows.as_str());

    literal.push_str("\n        ");

    literal
}
