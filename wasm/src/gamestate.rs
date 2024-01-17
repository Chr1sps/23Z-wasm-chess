use crate::{piece::Piece, Move, Player, Position, PromotionType};

type Field = Option<Piece>;
pub struct GameState {
    board: Vec<Vec<Field>>,
    current_player: Player,
}
impl GameState {
    /// Generates the next state that would be the result of a given move and a
    /// given promotion.
    pub fn generate_next_state(
        old_state: GameState,
        r#move: Move,
        promotion: Option<PromotionType>,
    ) -> Self {
        unimplemented!()
    }
    /// Creates an initial chessboard state.
    pub fn init() -> Self {
        Self {
            board: vec![
                vec![
                    Some(Piece::new_rook(
                        Position::new(0, 0).unwrap(),
                        Player::White,
                        true,
                    )),
                    Some(Piece::new_knight(
                        Position::new(0, 1).unwrap(),
                        Player::White,
                    )),
                    Some(Piece::new_bishop(
                        Position::new(0, 2).unwrap(),
                        Player::White,
                    )),
                    Some(Piece::new_queen(
                        Position::new(0, 3).unwrap(),
                        Player::White,
                    )),
                    Some(Piece::new_king(
                        Position::new(0, 4).unwrap(),
                        Player::White,
                        true,
                    )),
                    Some(Piece::new_bishop(
                        Position::new(0, 5).unwrap(),
                        Player::White,
                    )),
                    Some(Piece::new_knight(
                        Position::new(0, 6).unwrap(),
                        Player::White,
                    )),
                    Some(Piece::new_rook(
                        Position::new(0, 7).unwrap(),
                        Player::White,
                        true,
                    )),
                ],
                {
                    let mut result = vec![];
                    for i in 0..=7 as u8 {
                        result.push(Some(Piece::new_pawn(
                            Position::new(1, i).unwrap(),
                            Player::White,
                            true,
                            false,
                        )));
                    }
                    result
                },
                vec![],
                vec![],
                vec![],
                vec![],
                {
                    let mut result = vec![];
                    for i in 0..=7 as u8 {
                        result.push(Some(Piece::new_pawn(
                            Position::new(6, i).unwrap(),
                            Player::Black,
                            true,
                            false,
                        )));
                    }
                    result
                },
                vec![
                    Some(Piece::new_rook(
                        Position::new(7, 0).unwrap(),
                        Player::Black,
                        true,
                    )),
                    Some(Piece::new_knight(
                        Position::new(7, 1).unwrap(),
                        Player::Black,
                    )),
                    Some(Piece::new_bishop(
                        Position::new(7, 2).unwrap(),
                        Player::Black,
                    )),
                    Some(Piece::new_queen(
                        Position::new(7, 3).unwrap(),
                        Player::Black,
                    )),
                    Some(Piece::new_king(
                        Position::new(7, 4).unwrap(),
                        Player::Black,
                        true,
                    )),
                    Some(Piece::new_bishop(
                        Position::new(7, 5).unwrap(),
                        Player::Black,
                    )),
                    Some(Piece::new_knight(
                        Position::new(7, 6).unwrap(),
                        Player::Black,
                    )),
                    Some(Piece::new_rook(
                        Position::new(7, 7).unwrap(),
                        Player::Black,
                        true,
                    )),
                ],
            ],
            current_player: Player::White,
        }
    }

    /// Returns all the moves that can be made by the current player.
    pub fn get_all_moves(&self) -> Vec<Move> {
        let mut result = vec![];
        for field in self.board.iter().flatten() {
            if let Some(piece) = field {
                if piece.get_player() == self.current_player {
                    result.append(&mut piece.get_moves(&self));
                }
            }
        }
        result
    }

    /// Returns all the moves that can be made by a piece on a given position
    /// (returns an empty vector if there is no piece there).
    pub fn get_moves(&self, position: Position) -> Vec<Move> {
        let (row, col) = position.to_tuple();
        match &self.board[row as usize][col as usize] {
            Some(piece) => piece.get_moves(&self),
            None => vec![],
        }
    }

    /// Returns true if the current state indicates that the game has finished.
    pub fn is_finished(&self) -> bool {
        self.get_all_moves().is_empty()
    }

    /// If a match has resulted in a win, returns the winning player. Otherwise
    /// returns None.
    pub fn get_winner(&self) -> Option<Player> {
        unimplemented!()
    }
    /// Returns a reference to a piece if it exists there, otherwise returns
    /// None.
    pub fn get_piece(&self, position: Position) -> Option<&Piece> {
        let (row, col) = position.to_tuple();
        if let Some(piece) = &self.board[row as usize][col as usize] {
            Some(piece)
        } else {
            None
        }
    }
    /// Returns true if a given position resides on the last row from the
    /// perspective of the current player.
    fn is_end_row(&self, position: Position) -> bool {
        let end_row = if self.current_player == Player::White {
            7 as u8
        } else {
            0 as u8
        };
        position.get_row() == end_row
    }
    /// Returns true if a move would result in a promotion of a pawn.
    pub fn is_promotion_move(&self, r#move: Move) -> bool {
        let (end_row, col) = r#move.get_end_position().to_tuple();
        let moved_piece = &self.board[end_row as usize][col as usize];
        if let Some(Piece::Pawn(_, _, _)) = moved_piece {
            self.is_end_row(r#move.get_end_position())
        } else {
            false
        }
    }
}
