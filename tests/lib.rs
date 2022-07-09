mod common;

use chess::core::state::{PossibleMove, PossibleMoveKind};
use common::{from_literal, to_literal};
use std::vec;

#[test]
fn pawn_initial_move() {
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

    state.handle_action((6, 6));

    assert_eq!(
        state.possible_moves,
        vec![
            PossibleMove {
                kind: PossibleMoveKind::Move,
                coordinate: (6, 5)
            },
            PossibleMove {
                kind: PossibleMoveKind::Move,
                coordinate: (6, 4)
            }
        ]
    );

    state.handle_action((6, 4));

    assert_eq!(state.possible_moves, vec![]);
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
