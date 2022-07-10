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
4 . . . . . . . .
5 . . . . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
7 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖",
    );

    state.select_tile((1, 7));

    assert_eq!(
        state.possible_moves,
        vec![
            PossibleMove {
                kind: Move,
                coordinate: (0, 5)
            },
            PossibleMove {
                kind: Move,
                coordinate: (2, 5)
            },
        ]
    );

    state.select_tile((2, 5));

    assert_eq!(
        to_literal(&state),
        "
  0 1 2 3 4 5 6 7
0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
1 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
2 . . . . . . . .
3 . . . . . . . .
4 . . . . . . . .
5 . . ♘ . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
7 ♖ . ♗ ♕ ♔ ♗ ♘ ♖"
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
3 . ♘ . . . . . .
4 . . . . . . . .
5 . . . . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
7 ♖ . ♗ ♕ ♔ ♗ ♘ ♖",
    );

    state.select_tile((1, 3));

    assert_eq!(
        state.possible_moves,
        vec![
            PossibleMove {
                kind: Move,
                coordinate: (0, 5)
            },
            PossibleMove {
                kind: Capture,
                coordinate: (0, 1)
            },
            PossibleMove {
                kind: Move,
                coordinate: (2, 5)
            },
            PossibleMove {
                kind: Capture,
                coordinate: (2, 1)
            },
            PossibleMove {
                kind: Move,
                coordinate: (3, 4)
            },
            PossibleMove {
                kind: Move,
                coordinate: (3, 2)
            }
        ]
    );

    state.select_tile((2, 1));

    assert_eq!(
        to_literal(&state),
        "
  0 1 2 3 4 5 6 7
0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
1 ♟ ♟ ♘ ♟ ♟ ♟ ♟ ♟
2 . . . . . . . .
3 . . . . . . . .
4 . . . . . . . .
5 . . . . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
7 ♖ . ♗ ♕ ♔ ♗ ♘ ♖"
    );
}
