mod common;

use chess::core::state::{
    PossibleMove,
    PossibleMoveKind::{Capture, Move},
};
use common::{from_literal, to_literal};
use pretty_assertions::assert_eq;
use std::vec;

#[test]
fn possible_moves() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . ♔ ♟ . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ . ♗ ♘ ♖
        ",
    );

    state.select_tile((4, 3));

    assert_eq!(
        state.possible_moves,
        vec![
            PossibleMove {
                kind: Capture,
                coordinate: (5, 3),
            },
            PossibleMove {
                kind: Move,
                coordinate: (3, 3),
            },
            PossibleMove {
                kind: Move,
                coordinate: (5, 4),
            },
            PossibleMove {
                kind: Move,
                coordinate: (3, 4),
            }
        ]
    );
}

#[test]
fn basic_move() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . ♔ ♟ . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
        ",
    );

    state.select_tile((4, 3));
    state.select_tile((3, 3));

    assert_eq!(
        to_literal(&state),
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . ♔ . ♟ . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
        ",
    );
}

#[test]
fn capture() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . ♔ ♟ . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
        ",
    );

    state.select_tile((3, 4));
    state.select_tile((3, 5));

    assert_eq!(
        to_literal(&state),
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . ♔ ♟ . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
        "
    );
}
