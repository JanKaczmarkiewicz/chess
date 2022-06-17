#[derive(Clone)]

pub enum Chessman {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Eq, PartialEq, Clone)]
pub enum Side {
    White,
    Black,
}

#[derive(Eq, PartialEq, Debug)]
pub enum PossibleMoveKind {
    Move,
    Capture,
}

#[derive(Eq, PartialEq, Debug)]
pub struct PossibleMove {
    pub kind: PossibleMoveKind,
    pub coordinate: (usize, usize),
}

const BOARD_SIZE: usize = 8;

pub struct State {
    pub board: [[Option<(Chessman, Side)>; BOARD_SIZE]; BOARD_SIZE],
    pub selected_tile: Option<(usize, usize)>,
    pub current_side: Side,
    pub possible_moves: Vec<PossibleMove>,
}

impl State {
    pub fn new() -> Self {
        Self {
            current_side: Side::White,
            selected_tile: None,
            possible_moves: vec![],
            board: [
                [
                    Some((Chessman::Rook, Side::Black)),
                    Some((Chessman::Knight, Side::Black)),
                    Some((Chessman::Bishop, Side::Black)),
                    Some((Chessman::Queen, Side::Black)),
                    Some((Chessman::King, Side::Black)),
                    Some((Chessman::Bishop, Side::Black)),
                    Some((Chessman::Knight, Side::Black)),
                    Some((Chessman::Rook, Side::Black)),
                ],
                [
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                    Some((Chessman::Pawn, Side::Black)),
                ],
                [None, None, None, None, None, None, None, None],
                [
                    None,
                    None,
                    Some((Chessman::Rook, Side::White)),
                    None,
                    Some((Chessman::Bishop, Side::White)),
                    None,
                    None,
                    None,
                ],
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some((Chessman::Queen, Side::White)),
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                    Some((Chessman::Pawn, Side::White)),
                ],
                [
                    Some((Chessman::Rook, Side::White)),
                    Some((Chessman::Knight, Side::White)),
                    Some((Chessman::Bishop, Side::White)),
                    Some((Chessman::Queen, Side::White)),
                    Some((Chessman::King, Side::White)),
                    Some((Chessman::Bishop, Side::White)),
                    Some((Chessman::Knight, Side::White)),
                    Some((Chessman::Rook, Side::White)),
                ],
            ],
        }
    }

    fn is_coordinate_in_board((x, y): (i32, i32)) -> bool {
        x >= 0 && x < BOARD_SIZE as i32 && y >= 0 && y < BOARD_SIZE as i32
    }

    fn get_tile(&self, (x, y): (i32, i32)) -> Option<(Chessman, Side)> {
        if Self::is_coordinate_in_board((x, y)) {
            self.board[y as usize][x as usize]
                .as_ref()
                .map(|t| t.clone())
        } else {
            None
        }
    }

    fn assign_direction_possible_movies(
        &mut self,
        (x, y): (i32, i32),
        directions: &[(i32, i32)],
        (_, selected_tile_chessman_side): (Chessman, Side),
    ) {
        for (x_mod, y_mod) in directions {
            for i in 1..BOARD_SIZE {
                let x_next = x + x_mod * i as i32;
                let y_next = y + y_mod * i as i32;

                let coordinate = (x_next, y_next);

                if !Self::is_coordinate_in_board(coordinate) {
                    break;
                }

                if let Some((_, side)) = self.get_tile(coordinate) {
                    if selected_tile_chessman_side != side {
                        self.possible_moves.push(PossibleMove {
                            kind: PossibleMoveKind::Capture,
                            coordinate: (coordinate.0 as usize, coordinate.1 as usize),
                        });
                    }

                    break;
                } else {
                    self.possible_moves.push(PossibleMove {
                        kind: PossibleMoveKind::Move,
                        coordinate: (coordinate.0 as usize, coordinate.1 as usize),
                    });
                }
            }
        }
    }

    fn assign_bishop_possible_movies(&mut self, input: (i32, i32), chessman: (Chessman, Side)) {
        self.assign_direction_possible_movies(
            input,
            &[(1, 1), (1, -1), (-1, 1), (-1, -1)],
            chessman,
        )
    }

    fn assign_rook_possible_movies(&mut self, input: (i32, i32), chessman: (Chessman, Side)) {
        self.assign_direction_possible_movies(input, &[(1, 0), (0, 1), (0, -1), (-1, 0)], chessman)
    }

    fn assign_queen_possible_movies(&mut self, input: (i32, i32), chessman: (Chessman, Side)) {
        self.assign_direction_possible_movies(
            input,
            &[
                (1, 0),
                (0, 1),
                (0, -1),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ],
            chessman,
        )
    }

    pub fn handle_action(&mut self, input: (i32, i32)) {
        self.possible_moves = vec![];

        if let Some((chessman, side)) = self.get_tile(input) {
            if side == self.current_side {
                match chessman {
                    Chessman::Bishop => self.assign_bishop_possible_movies(input, (chessman, side)),
                    Chessman::Rook => self.assign_rook_possible_movies(input, (chessman, side)),
                    Chessman::Queen => self.assign_queen_possible_movies(input, (chessman, side)),
                    _ => {}
                }

                self.selected_tile = Some((input.0 as usize, input.1 as usize));

                return;
            }
        };
        self.selected_tile = None;
    }
}
