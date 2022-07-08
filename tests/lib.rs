use std::vec;

use chess::core::state::{PossibleMove, PossibleMoveKind, State};

#[test]
fn pawn_initial_move() {
    let mut state = State::new();

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
    assert!(state.get_board()[4][6].is_some());
    assert!(state.get_board()[6][6].is_none());
}
