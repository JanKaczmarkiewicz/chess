mod common;

use common::from_literal;

#[test]
fn basic_checkmate() {
    let state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ . ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ . ♟ ♟ ♟
        2 . . . . . . . .
        3 . . . . ♟ . . .
        4 . . . . . ♙ ♙ ♛
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ ♙ . . ♙
        7 ♖ ♗ ♘ ♕ ♔ ♗ ♘ ♖
        ",
    );

    assert!(state.is_checkmate());
}

#[test]
fn covering() {
    let state = from_literal(
        "
          0 1 2 3 4 5 6 7
        0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        1 ♟ ♟ ♟ ♟ ♟ . ♟ ♟
        2 . . . . . . . .
        3 . . . . . ♟ . ♕
        4 . . . . ♙ . . .
        5 . . . . . . . .
        6 ♙ ♙ ♙ ♙ . ♙ ♙ ♙
        7 ♖ ♗ ♘ . ♔ ♗ ♘ ♖
        ",
    );

    assert!(!state.is_checkmate());
}
