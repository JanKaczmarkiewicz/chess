mod common;

use chess::core::state::{
    PossibleMove,
    PossibleMoveKind::{Capture, Move},
};
use common::{from_literal, to_literal};
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
4 . . . ♖ . . . .
5 . . . . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
7 . ♘ ♗ ♕ ♔ ♗ ♘ ♖",
    );

    state.select_tile((3, 4));

    assert_eq!(
        state.possible_moves,
        vec![
            PossibleMove {
                kind: Move,
                coordinate: (4, 4)
            },
            PossibleMove {
                kind: Move,
                coordinate: (5, 4)
            },
            PossibleMove {
                kind: Move,
                coordinate: (6, 4)
            },
            PossibleMove {
                kind: Move,
                coordinate: (7, 4)
            },
            PossibleMove {
                kind: Move,
                coordinate: (3, 5)
            },
            PossibleMove {
                kind: Move,
                coordinate: (3, 3)
            },
            PossibleMove {
                kind: Move,
                coordinate: (3, 2)
            },
            PossibleMove {
                kind: Capture,
                coordinate: (3, 1)
            },
            PossibleMove {
                kind: Move,
                coordinate: (2, 4)
            },
            PossibleMove {
                kind: Move,
                coordinate: (1, 4)
            },
            PossibleMove {
                kind: Move,
                coordinate: (0, 4)
            }
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
4 . . . . . . ♖ .
5 . . . . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
7 . ♘ ♗ ♕ ♔ ♗ ♘ ♖"
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
4 . . . ♖ . . . .
5 . . . . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
7 . ♘ ♗ ♕ ♔ ♗ ♘ ♖",
    );

    state.select_tile((3, 4));
    state.select_tile((3, 1));

    assert_eq!(
        to_literal(&state),
        "
  0 1 2 3 4 5 6 7
0 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
1 ♟ ♟ ♟ ♖ ♟ ♟ ♟ ♟
2 . . . . . . . .
3 . . . . . . . .
4 . . . . . . . .
5 . . . . . . . .
6 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
7 . ♘ ♗ ♕ ♔ ♗ ♘ ♖"
    );
}
