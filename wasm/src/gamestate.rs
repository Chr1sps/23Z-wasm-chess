use crate::{piece::Piece, Move, Player, Position, PromotionType};

type Field = Option<Piece>;
type Board = Vec<Vec<Field>>;
pub struct GameState {
    /// The main board structure that contains all the fields.
    board: Board,
    /// The player whose move it is now.
    current_player: Player,
    /// If there is a pawn that can be taken en passant, it's position is
    /// denoted here. There is always at most a single pawn that can be taken
    /// en passant.
    en_passant_square: Option<Position>,
}

macro_rules! make_board {
    ($($piece:expr),* $(,)?) => {{
        let mut _temp_board: Board = vec![vec![None; 8]; 8];
        $(
            let (row_, col_) = $piece.get_position().as_tuple();
            _temp_board[row_ as usize][col_ as usize] = Some($piece);
        )*
        _temp_board
    }};
}
// macro_rules! make_pawn_row {
//     ($row:expr, $player:expr) => {
//         Piece::new_pawn(Position::new($row, $i).unwrap(), $player, true),
//         Piece::new_pawn(Position::new($row, 1).unwrap(), $player, true),
//         Piece::new_pawn(Position::new($row, 2).unwrap(), $player, true),
//         Piece::new_pawn(Position::new($row, 3).unwrap(), $player, true),
//         Piece::new_pawn(Position::new($row, 4).unwrap(), $player, true),
//         Piece::new_pawn(Position::new($row, 5).unwrap(), $player, true),
//         Piece::new_pawn(Position::new($row, 6).unwrap(), $player, true),
//         Piece::new_pawn(Position::new($row, 7).unwrap(), $player, true)
//     };
// }
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
            board: make_board!(
                Piece::new_rook(Position::new(0, 0).unwrap(), Player::White, true,),
                Piece::new_knight(Position::new(0, 1).unwrap(), Player::White,),
                Piece::new_bishop(Position::new(0, 2).unwrap(), Player::White,),
                Piece::new_queen(Position::new(0, 3).unwrap(), Player::White,),
                Piece::new_king(Position::new(0, 4).unwrap(), Player::White, true,),
                Piece::new_bishop(Position::new(0, 5).unwrap(), Player::White,),
                Piece::new_knight(Position::new(0, 6).unwrap(), Player::White,),
                Piece::new_rook(Position::new(0, 7).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(1, 0).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(1, 1).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(1, 2).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(1, 3).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(1, 4).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(1, 5).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(1, 6).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(1, 7).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(6, 0).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(6, 1).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(6, 2).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(6, 3).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(6, 4).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(6, 5).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(6, 6).unwrap(), Player::White, true,),
                Piece::new_pawn(Position::new(6, 7).unwrap(), Player::White, true,),
                Piece::new_knight(Position::new(7, 1).unwrap(), Player::Black,),
                Piece::new_bishop(Position::new(7, 2).unwrap(), Player::Black,),
                Piece::new_queen(Position::new(7, 3).unwrap(), Player::Black,),
                Piece::new_king(Position::new(7, 4).unwrap(), Player::Black, true,),
                Piece::new_bishop(Position::new(7, 5).unwrap(), Player::Black,),
                Piece::new_knight(Position::new(7, 6).unwrap(), Player::Black,),
                Piece::new_rook(Position::new(7, 7).unwrap(), Player::Black, true,),
            ),
            current_player: Player::White,
            en_passant_square: None,
        }
    }
    /// Creates a state from a given board and other data.
    pub fn from_board(
        init_board: Board,
        player: Player,
        en_passant_square: Option<Position>,
    ) -> Option<Self> {
        if init_board.len() == 8 && init_board.iter().all(|vec| vec.len() == 8) {
            Some(Self {
                board: init_board,
                current_player: player,
                en_passant_square,
            })
        } else {
            None
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
        let (row, col) = position.as_tuple();
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
        let moves = self.get_all_moves();
        if moves.is_empty() {
            Some(Player::White)
        } else {
            None
        }
    }
    /// Returns a reference to a piece if it exists there, otherwise returns
    /// None.
    pub fn get_piece(&self, position: Position) -> Option<&Piece> {
        let (row, col) = position.as_tuple();
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
        let (end_row, col) = r#move.get_end_position().as_tuple();
        let moved_piece = &self.board[end_row as usize][col as usize];
        if let Some(Piece::Pawn(_, _)) = moved_piece {
            self.is_end_row(r#move.get_end_position())
        } else {
            false
        }
    }
    pub fn get_en_passant_square(&self) -> Option<Position> {
        self.en_passant_square
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn test_get_moves(state: GameState, position: Position, expected_moves: Vec<Move>) {
        let moves = state.get_moves(position);
        assert_eq!(moves, expected_moves);
    }
    macro_rules! make_move {
        ($a:expr, $b:expr, $c:expr, $d:expr) => {
            Move::new(
                Position::new($a, $b).unwrap(),
                Position::new($c, $d).unwrap(),
            )
        };
    }
    macro_rules! make_pos {
        ($a:expr, $b:expr) => {
            Position::new($a, $b).unwrap()
        };
    }
    #[test]
    fn test_init_pawn_moves() {
        let board = make_board!(Piece::new_pawn(make_pos!(1, 0), Player::White, true));
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(1, 0);
        let expected_moves = vec![make_move!(1, 0, 2, 0), make_move!(1, 0, 3, 0)];
        test_get_moves(state, pos, expected_moves);
    }
    #[test]
    fn test_pawn_non_first_moves() {
        let board = make_board!(Piece::new_pawn(make_pos!(2, 0), Player::White, false));
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(2, 0);
        let expected_moves = vec![make_move!(2, 0, 3, 0)];
        test_get_moves(state, pos, expected_moves);
    }
    #[test]
    fn test_pawn_blocked_by_friendly() {
        let board = make_board!(
            Piece::new_pawn(make_pos!(2, 0), Player::White, false),
            Piece::new_pawn(make_pos!(3, 0), Player::White, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(2, 0);
        let expected_moves = vec![];
        test_get_moves(state, pos, expected_moves);
    }
    #[test]
    fn test_pawn_blocked_by_enemy() {
        let board = make_board!(
            Piece::new_pawn(make_pos!(2, 0), Player::White, false),
            Piece::new_pawn(make_pos!(3, 0), Player::Black, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(2, 0);
        let expected_moves = vec![];
        test_get_moves(state, pos, expected_moves);
    }
}
