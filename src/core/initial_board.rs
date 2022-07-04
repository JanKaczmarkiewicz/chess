use super::{
    chessman::chessman::{Chessman, ChessmanKind},
    state::{Side, Tiles},
};

pub const INITIAL_BOARD: Tiles = [
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
];
