mod common;

use chess::core::state::{
    PossibleMove,
    PossibleMoveKind::{Capture, Move},
};
use common::{from_literal, to_literal};
use pretty_assertions::assert_eq;
use std::vec;

#[test]
fn basic_move() {
    let mut state = from_literal(
        "
  0 1 2 3 4 5 6 7
0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
2 . . . . . . . .
3 . . . . . . . .
4 . . . . . . ♙ .
5 . . . . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ . ♙
7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖",
    );

    state.select_tile((6, 4));

    assert_eq!(
        state.possible_moves,
        vec![PossibleMove {
            kind: Move,
            coordinate: (6, 3)
        },]
    );

    state.select_tile((6, 3));

    assert_eq!(
        to_literal(&state),
        "
  0 1 2 3 4 5 6 7
0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
2 . . . . . . . .
3 . . . . . . ♙ .
4 . . . . . . . .
5 . . . . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ . ♙
7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖"
    );
}

#[test]
fn initial_move() {
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
7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖",
    );

    state.select_tile((6, 6));

    assert_eq!(
        state.possible_moves,
        vec![
            PossibleMove {
                kind: Move,
                coordinate: (6, 5)
            },
            PossibleMove {
                kind: Move,
                coordinate: (6, 4)
            },
        ]
    );

    state.select_tile((6, 4));

    assert_eq!(
        to_literal(&state),
        "
  0 1 2 3 4 5 6 7
0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
2 . . . . . . . .
3 . . . . . . . .
4 . . . . . . ♙ .
5 . . . . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ . ♙
7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖"
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
3 . . . . . . . .
4 . . . . . . . .
5 . . . . . ♟ . .
6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖",
    );

    state.select_tile((6, 6));

    assert_eq!(
        state.possible_moves,
        vec![
            PossibleMove {
                kind: Move,
                coordinate: (6, 5)
            },
            PossibleMove {
                kind: Move,
                coordinate: (6, 4)
            },
            PossibleMove {
                kind: Capture,
                coordinate: (5, 5)
            },
        ]
    );

    state.select_tile((5, 5));

    assert_eq!(state.possible_moves, vec![]);
    assert_eq!(
        to_literal(&state),
        "
  0 1 2 3 4 5 6 7
0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
2 . . . . . . . .
3 . . . . . . . .
4 . . . . . . . .
5 . . . . . ♙ . .
6 ♙ ♙ ♙ ♙ ♙ ♙ . ♙
7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖"
    );
}
