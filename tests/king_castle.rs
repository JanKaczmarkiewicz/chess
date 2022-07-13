mod common;

use common::{from_literal, to_literal};
use pretty_assertions::assert_eq;

#[test]
fn castle_left() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . . . . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ . . . ♔ ♗ ♘ ♖
        ",
    );

    state.select_tile((4, 7));
    state.select_tile((2, 7));

    assert_eq!(
        to_literal(&state),
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . . . . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 . . ♔ ♖ . ♗ ♘ ♖
        "
    );
}

#[test]
fn castle_left_king_moved() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . . . . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ . . . ♔ ♗ ♘ ♖
        ",
    );

    // white king move
    state.select_tile((4, 7));
    state.select_tile((3, 7));

    // black move
    state.select_tile((6, 1));
    state.select_tile((6, 2));

    // white king move (return to initial position)
    state.select_tile((3, 7));
    state.select_tile((4, 7));

    // black move
    state.select_tile((6, 2));
    state.select_tile((6, 3));

    // white king tries castle
    state.select_tile((4, 7));
    state.select_tile((2, 7));

    assert_eq!(
        to_literal(&state),
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ . ♟
        2 . . . . . . . .
        3 . . . . . . ♟ .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ . . . ♔ ♗ ♘ ♖
        "
    );
}

#[test]
fn castle_left_path_check() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . ♛ . . . . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ . ♙ ♙ ♙ ♙ ♙
        7 ♖ . . . ♔ ♗ ♘ ♖
        ",
    );

    state.select_tile((4, 7));
    state.select_tile((2, 7));

    assert_eq!(
        to_literal(&state),
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . ♛ . . . . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ . ♙ ♙ ♙ ♙ ♙
        7 ♖ . . . ♔ ♗ ♘ ♖
        "
    );
}

#[test]
fn castle_left_rook_moved() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . . . . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ . . . ♔ ♗ ♘ ♖
        ",
    );

    // white rook move
    state.select_tile((0, 7));
    state.select_tile((1, 7));

    // black move
    state.select_tile((6, 1));
    state.select_tile((6, 2));

    // white rook move (return to initial position)
    state.select_tile((1, 7));
    state.select_tile((0, 7));

    // black move
    state.select_tile((6, 2));
    state.select_tile((6, 3));

    // white king tries castle
    state.select_tile((4, 7));
    state.select_tile((2, 7));

    assert_eq!(
        to_literal(&state),
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ . ♟
        2 . . . . . . . .
        3 . . . . . . ♟ .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ . . . ♔ ♗ ♘ ♖
        "
    );
}

#[test]
fn castle_right() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . . . . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ ♔ . . ♖
        ",
    );

    state.select_tile((4, 7));
    state.select_tile((6, 7));

    assert_eq!(
        to_literal(&state),
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . . . . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ . ♖ ♔ .
        "
    );
}

#[test]
fn castle_right_path_check() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . . . ♛ .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ . ♙
        7 ♖ ♘ ♗ ♕ ♔ . . ♖
        ",
    );

    state.select_tile((4, 7));
    state.select_tile((6, 7));

    assert_eq!(
        to_literal(&state),
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . . . ♛ .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ . ♙
        7 ♖ ♘ ♗ ♕ ♔ . . ♖
        "
    );
}

#[test]
fn castle_right_king_moved() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . . . . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ ♔ . . ♖
        ",
    );

    // white king move
    state.select_tile((4, 7));
    state.select_tile((5, 7));

    // black move
    state.select_tile((6, 1));
    state.select_tile((6, 2));

    // white king move (return to initial position)
    state.select_tile((5, 7));
    state.select_tile((4, 7));

    // black move
    state.select_tile((6, 2));
    state.select_tile((6, 3));

    // white king tries castle
    state.select_tile((4, 7));
    state.select_tile((6, 7));

    assert_eq!(
        to_literal(&state),
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ . ♟
        2 . . . . . . . .
        3 . . . . . . ♟ .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ ♔ . . ♖
        "
    );
}

#[test]
fn castle_right_rook_moved() {
    let mut state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . . . . .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ ♔ . . ♖
        ",
    );

    // white king move
    state.select_tile((7, 7));
    state.select_tile((6, 7));

    // black move
    state.select_tile((6, 1));
    state.select_tile((6, 2));

    // white rook move (return to initial position)
    state.select_tile((6, 7));
    state.select_tile((7, 7));

    // black move
    state.select_tile((6, 2));
    state.select_tile((6, 3));

    // white king tries castle
    state.select_tile((4, 7));
    state.select_tile((6, 7));

    assert_eq!(
        to_literal(&state),
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ ♟ . ♟
        2 . . . . . . . .
        3 . . . . . . ♟ .
        4 . . . . . . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        7 ♖ ♘ ♗ ♕ ♔ . . ♖
        "
    );
}
