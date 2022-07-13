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
