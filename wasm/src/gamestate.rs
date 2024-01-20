use crate::{piece::Piece, player::get_opponent, Move, PieceType, Player, Position, PromotionType};

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
impl GameState {
    pub fn update_en_passant(state: &GameState, r#move: &Move) -> Option<Position> {
        let en_passant_square: Option<Position>;
        // Sprawdź, czy ostatni ruch był wykonany przez pionka, który przesunął się o dwa pola do przodu
        let (start_row, start_col) = r#move.get_current_position().as_tuple();
        let (end_row, end_col) = r#move.get_end_position().as_tuple();
        if let Some(Piece::Pawn(data, _)) = state.board[start_row as usize][start_col as usize] {
            if (end_row as i32 - start_row as i32).abs() == 2 {
                // Sprawdź, czy istnieje pionek przeciwnika, który może wykonać ruch "en passant"
                for &col in &[end_col - 1, end_col + 1] {
                    if let Some(Piece::Pawn(other_data, _)) =
                        state.board[end_row as usize][col as usize]
                    {
                        if data.get_player() != other_data.get_player() {
                            // Ustaw `en_passant_square` na pozycję pionka, który może być zniszczony
                            en_passant_square = Some(Position::new(end_row, end_col).unwrap());
                            return en_passant_square;
                        }
                    }
                }
            }
        }
        // Jeśli żadne z powyższych warunków nie zostało spełnione, ustaw `en_passant_square` na `None`
        en_passant_square = None;
        return en_passant_square;
    }
    /// Generates the next state that would be the result of a given move and a
    /// given promotion. This method does NOT check the legality of a given
    /// move, but only performs it. If a promotion is specified, the moved
    /// piece is substituted with an appropriate new one.
    pub fn transform_state(
        state: &GameState,
        r#move: Move,
        promotion: Option<PromotionType>,
    ) -> Self {
        let mut new_board = state.board.clone();
        let (start_row, start_col) = r#move.get_current_position().as_tuple();
        let (end_row, end_col) = r#move.get_end_position().as_tuple();
        let moved_piece = new_board[start_row as usize][start_col as usize].take();
        new_board[end_row as usize][end_col as usize] = moved_piece;
        if let Some(promotion_type) = promotion {
            let (row, col) = r#move.get_end_position().as_tuple();
            let promoted_piece = match promotion_type {
                PromotionType::Knight => {
                    Piece::new_knight(Position::new(row, col).unwrap(), state.current_player)
                }
                PromotionType::Bishop => {
                    Piece::new_bishop(Position::new(row, col).unwrap(), state.current_player)
                }
                PromotionType::Rook => {
                    Piece::new_rook(Position::new(row, col).unwrap(), state.current_player, true)
                }
                PromotionType::Queen => {
                    Piece::new_queen(Position::new(row, col).unwrap(), state.current_player)
                }
            };
            new_board[row as usize][col as usize] = Some(promoted_piece);
        }
        Self {
            board: new_board,
            current_player: get_opponent(state.current_player),
            en_passant_square: GameState::update_en_passant(&state, &r#move),
        }
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
                Piece::new_pawn(Position::new(6, 0).unwrap(), Player::Black, true,),
                Piece::new_pawn(Position::new(6, 1).unwrap(), Player::Black, true,),
                Piece::new_pawn(Position::new(6, 2).unwrap(), Player::Black, true,),
                Piece::new_pawn(Position::new(6, 3).unwrap(), Player::Black, true,),
                Piece::new_pawn(Position::new(6, 4).unwrap(), Player::Black, true,),
                Piece::new_pawn(Position::new(6, 5).unwrap(), Player::Black, true,),
                Piece::new_pawn(Position::new(6, 6).unwrap(), Player::Black, true,),
                Piece::new_pawn(Position::new(6, 7).unwrap(), Player::Black, true,),
                Piece::new_rook(Position::new(7, 0).unwrap(), Player::Black, true,),
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

    /// Returns all the moves that can be made by a piece on a given position
    /// (returns an empty vector if there is no piece there).
    pub fn get_moves(&self, position: Position) -> Vec<Move> {
        let (row, col) = position.as_tuple();
        match &self.board[row as usize][col as usize] {
            Some(piece) => piece.get_moves(&self),
            None => vec![],
        }
    }

    /// Returns all the legal moves that can be made by the current player.
    fn get_all_moves(&self) -> Vec<Move> {
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

    /// Returns all the legal moves that can be made by the other player.
    fn get_all_opponent_moves(&self) -> Vec<Move> {
        let mut result = vec![];
        for field in self.board.iter().flatten() {
            if let Some(piece) = field {
                if piece.get_player() != self.current_player {
                    result.append(&mut piece.get_moves(&self));
                }
            }
        }
        result
    }

    /// Returns true if the current state indicates that the game has finished.
    pub fn is_finished(&self) -> bool {
        self.get_all_moves().is_empty()
    }

    /// Returns true if the current player is under check.
    fn is_checked(&self) -> bool {
        let king_position = self
            .board
            .iter()
            .flatten()
            .find(|field| {
                field
                    .as_ref()
                    .map(|piece| {
                        piece.get_player() == self.current_player
                            && (piece.get_type() == PieceType::King)
                    })
                    .unwrap_or(false)
            })
            .unwrap()
            .unwrap()
            .get_position();
        self.get_all_opponent_moves()
            .iter()
            .any(|r#move| r#move.get_end_position() == king_position)
    }

    /// If a match has resulted in a win, returns the winning player. Otherwise
    /// returns None.
    pub fn get_winner(&self) -> Option<Player> {
        if self.is_checked() {
            Some(get_opponent(self.current_player))
        } else {
            None
        }
    }
    /// Returns a reference to a piece if it exists there, otherwise returns
    /// None.
    pub fn get_piece(&self, position: Position) -> Option<&Piece> {
        let (row, col) = position.as_tuple();
        self.board[row as usize][col as usize].as_ref()
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
        let (start_row, start_col) = r#move.get_current_position().as_tuple();
        let end_pos = r#move.get_end_position();
        let (end_row, col) = end_pos.as_tuple();
        let moved_piece = &self.board[start_row as usize][start_col as usize];
        if let Some(Piece::Pawn(_, _)) = moved_piece {
            self.is_end_row(end_pos)
        } else {
            false
        }
    }
    pub fn get_en_passant_square(&self) -> Option<&Position> {
        self.en_passant_square.as_ref()
    }
    pub fn can_en_passant(game_state: &GameState, r#move: &Move) -> bool {
        let (start_row, start_col) = r#move.get_current_position().as_tuple();
        if let Some(Piece::Pawn(_, can_be_taken_en_passant)) =
            game_state.board[start_row as usize][start_col as usize]
        {
            if can_be_taken_en_passant {
                let (end_row, end_col) = r#move.get_end_position().as_tuple();
                if (end_row as i32 - start_row as i32).abs() == 1
                    && (end_col as i32 - start_col as i32).abs() == 1
                {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn test_get_moves(state: GameState, position: Position, expected_moves: &mut Vec<Move>) {
        let mut moves = state.get_moves(position);

        expected_moves.sort();
        moves.sort();
        assert_eq!(moves, *expected_moves);
    }
    fn check_move_not_in_get_moves(state: &GameState, position: &Position, checked_move: &Move) {
        let moves = state.get_moves(*position);
        assert!(moves.contains(checked_move));
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
        let mut expected_moves = vec![make_move!(1, 0, 2, 0), make_move!(1, 0, 3, 0)];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_pawn_non_first_moves() {
        let board = make_board!(Piece::new_pawn(make_pos!(2, 0), Player::White, false));
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(2, 0);
        let mut expected_moves = vec![make_move!(2, 0, 3, 0)];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_pawn_blocked_by_friendly() {
        let board = make_board!(
            Piece::new_pawn(make_pos!(2, 0), Player::White, false),
            Piece::new_pawn(make_pos!(3, 0), Player::White, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(2, 0);
        let mut expected_moves = vec![];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_pawn_blocked_by_enemy() {
        let board = make_board!(
            Piece::new_pawn(make_pos!(2, 0), Player::White, false),
            Piece::new_pawn(make_pos!(3, 0), Player::Black, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(2, 0);
        let mut expected_moves = vec![];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_pawn_can_take() {
        let board = make_board!(
            Piece::new_pawn(make_pos!(4, 3), Player::White, false),
            Piece::new_pawn(make_pos!(5, 4), Player::Black, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(4, 3);
        let mut expected_moves = vec![make_move!(4, 3, 5, 3), make_move!(4, 3, 5, 4)];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_pawn_en_passant() {
        let board = make_board!(
            Piece::new_pawn(make_pos!(2, 4), Player::White, false),
            Piece::new_pawn(make_pos!(4, 5), Player::Black, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let new_move = make_move!(2, 4, 4, 4);
        let new_state = GameState::transform_state(&state, new_move, None);
        println!("{:?}", state.get_piece(Position::new(4, 4).unwrap()));
        let result = GameState::update_en_passant(&state, &new_move);
        let mut expected_moves = vec![make_move!(4, 5, 3, 4), make_move!(4, 5, 3, 5)];
        let pos = make_pos!(4, 5);
        test_get_moves(new_state, pos, &mut expected_moves);
    }
    #[test]
    fn test_knight_moves_center() {
        let board = make_board!(Piece::new_knight(make_pos!(4, 3), Player::White),);
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(4, 3);
        let mut expected_moves = vec![
            make_move!(4, 3, 2, 2),
            make_move!(4, 3, 3, 1),
            make_move!(4, 3, 5, 1),
            make_move!(4, 3, 6, 2),
            make_move!(4, 3, 6, 4),
            make_move!(4, 3, 5, 5),
            make_move!(4, 3, 3, 5),
            make_move!(4, 3, 2, 4),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_knight_moves_corner() {
        let board = make_board!(Piece::new_knight(make_pos!(0, 0), Player::White),);
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(0, 0);
        let mut expected_moves = vec![make_move!(0, 0, 1, 2), make_move!(0, 0, 2, 1)];
        test_get_moves(state, pos, &mut expected_moves);
    }

    #[test]
    fn test_knight_can_take() {
        let board = make_board!(
            Piece::new_knight(make_pos!(0, 0), Player::White),
            Piece::new_pawn(make_pos!(1, 2), Player::Black, false),
            Piece::new_pawn(make_pos!(2, 1), Player::Black, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(0, 0);
        let mut expected_moves = vec![make_move!(0, 0, 1, 2), make_move!(0, 0, 2, 1)];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_knight_blocked() {
        let board = make_board!(
            Piece::new_knight(make_pos!(0, 0), Player::White),
            Piece::new_pawn(make_pos!(1, 2), Player::White, true),
            Piece::new_pawn(make_pos!(2, 1), Player::White, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(0, 0);
        let mut expected_moves = vec![];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_bishop_moves() {
        let board = make_board!(Piece::new_bishop(make_pos!(3, 3), Player::White),);
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 0, 0),
            make_move!(3, 3, 1, 1),
            make_move!(3, 3, 2, 2),
            make_move!(3, 3, 4, 4),
            make_move!(3, 3, 5, 5),
            make_move!(3, 3, 6, 6),
            make_move!(3, 3, 7, 7),
            make_move!(3, 3, 6, 0),
            make_move!(3, 3, 5, 1),
            make_move!(3, 3, 4, 2),
            make_move!(3, 3, 2, 4),
            make_move!(3, 3, 1, 5),
            make_move!(3, 3, 0, 6),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_bishop_can_take() {
        let board = make_board!(
            Piece::new_bishop(make_pos!(3, 3), Player::White),
            Piece::new_bishop(make_pos!(1, 1), Player::Black),
            Piece::new_bishop(make_pos!(1, 5), Player::Black),
            Piece::new_bishop(make_pos!(5, 5), Player::Black),
            Piece::new_bishop(make_pos!(5, 1), Player::Black),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 1, 1),
            make_move!(3, 3, 2, 2),
            make_move!(3, 3, 4, 4),
            make_move!(3, 3, 5, 5),
            make_move!(3, 3, 5, 1),
            make_move!(3, 3, 4, 2),
            make_move!(3, 3, 2, 4),
            make_move!(3, 3, 1, 5),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_bishop_blocked() {
        let board = make_board!(
            Piece::new_bishop(make_pos!(3, 3), Player::White),
            Piece::new_bishop(make_pos!(1, 1), Player::White),
            Piece::new_bishop(make_pos!(1, 5), Player::White),
            Piece::new_bishop(make_pos!(5, 5), Player::White),
            Piece::new_bishop(make_pos!(5, 1), Player::White),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 2, 2),
            make_move!(3, 3, 4, 4),
            make_move!(3, 3, 4, 2),
            make_move!(3, 3, 2, 4),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_rook_moves() {
        let board = make_board!(Piece::new_rook(make_pos!(3, 3), Player::White, false),);
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 3, 0),
            make_move!(3, 3, 3, 1),
            make_move!(3, 3, 3, 2),
            make_move!(3, 3, 3, 4),
            make_move!(3, 3, 3, 5),
            make_move!(3, 3, 3, 6),
            make_move!(3, 3, 3, 7),
            make_move!(3, 3, 0, 3),
            make_move!(3, 3, 1, 3),
            make_move!(3, 3, 2, 3),
            make_move!(3, 3, 4, 3),
            make_move!(3, 3, 5, 3),
            make_move!(3, 3, 6, 3),
            make_move!(3, 3, 7, 3),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_rook_can_take() {
        let board = make_board!(
            Piece::new_rook(make_pos!(3, 3), Player::White, false),
            Piece::new_rook(make_pos!(1, 3), Player::Black, false),
            Piece::new_rook(make_pos!(3, 1), Player::Black, false),
            Piece::new_rook(make_pos!(5, 3), Player::Black, false),
            Piece::new_rook(make_pos!(3, 5), Player::Black, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 3, 1),
            make_move!(3, 3, 3, 2),
            make_move!(3, 3, 3, 4),
            make_move!(3, 3, 3, 5),
            make_move!(3, 3, 1, 3),
            make_move!(3, 3, 2, 3),
            make_move!(3, 3, 4, 3),
            make_move!(3, 3, 5, 3),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_rook_blocked() {
        let board = make_board!(
            Piece::new_rook(make_pos!(3, 3), Player::White, false),
            Piece::new_rook(make_pos!(1, 3), Player::White, false),
            Piece::new_rook(make_pos!(3, 1), Player::White, false),
            Piece::new_rook(make_pos!(5, 3), Player::White, false),
            Piece::new_rook(make_pos!(3, 5), Player::White, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 3, 2),
            make_move!(3, 3, 3, 4),
            make_move!(3, 3, 2, 3),
            make_move!(3, 3, 4, 3),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_queen_moves() {
        let board = make_board!(Piece::new_queen(make_pos!(3, 3), Player::White),);
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 0, 0),
            make_move!(3, 3, 1, 1),
            make_move!(3, 3, 2, 2),
            make_move!(3, 3, 4, 4),
            make_move!(3, 3, 5, 5),
            make_move!(3, 3, 6, 6),
            make_move!(3, 3, 7, 7),
            make_move!(3, 3, 6, 0),
            make_move!(3, 3, 5, 1),
            make_move!(3, 3, 4, 2),
            make_move!(3, 3, 2, 4),
            make_move!(3, 3, 1, 5),
            make_move!(3, 3, 0, 6),
            make_move!(3, 3, 3, 0),
            make_move!(3, 3, 3, 1),
            make_move!(3, 3, 3, 2),
            make_move!(3, 3, 3, 4),
            make_move!(3, 3, 3, 5),
            make_move!(3, 3, 3, 6),
            make_move!(3, 3, 3, 7),
            make_move!(3, 3, 0, 3),
            make_move!(3, 3, 1, 3),
            make_move!(3, 3, 2, 3),
            make_move!(3, 3, 4, 3),
            make_move!(3, 3, 5, 3),
            make_move!(3, 3, 6, 3),
            make_move!(3, 3, 7, 3),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_queen_can_take() {
        let board = make_board!(
            Piece::new_queen(make_pos!(3, 3), Player::White),
            Piece::new_queen(make_pos!(2, 3), Player::Black),
            Piece::new_queen(make_pos!(2, 2), Player::Black),
            Piece::new_queen(make_pos!(3, 2), Player::Black),
            Piece::new_queen(make_pos!(4, 2), Player::Black),
            Piece::new_queen(make_pos!(4, 3), Player::Black),
            Piece::new_queen(make_pos!(4, 4), Player::Black),
            Piece::new_queen(make_pos!(3, 4), Player::Black),
            Piece::new_queen(make_pos!(2, 4), Player::Black),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 2, 2),
            make_move!(3, 3, 4, 4),
            make_move!(3, 3, 4, 2),
            make_move!(3, 3, 2, 4),
            make_move!(3, 3, 3, 2),
            make_move!(3, 3, 3, 4),
            make_move!(3, 3, 2, 3),
            make_move!(3, 3, 4, 3),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_queen_blocked() {
        let board = make_board!(
            Piece::new_queen(make_pos!(3, 3), Player::White),
            Piece::new_queen(make_pos!(2, 3), Player::White),
            Piece::new_queen(make_pos!(2, 2), Player::White),
            Piece::new_queen(make_pos!(3, 2), Player::White),
            Piece::new_queen(make_pos!(4, 2), Player::White),
            Piece::new_queen(make_pos!(4, 3), Player::White),
            Piece::new_queen(make_pos!(4, 4), Player::White),
            Piece::new_queen(make_pos!(3, 4), Player::White),
            Piece::new_queen(make_pos!(2, 4), Player::White),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_king_moves() {
        let board = make_board!(Piece::new_king(make_pos!(3, 3), Player::White, false),);
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 2, 2),
            make_move!(3, 3, 4, 4),
            make_move!(3, 3, 4, 2),
            make_move!(3, 3, 2, 4),
            make_move!(3, 3, 3, 2),
            make_move!(3, 3, 3, 4),
            make_move!(3, 3, 2, 3),
            make_move!(3, 3, 4, 3),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_king_can_take() {
        let board = make_board!(
            Piece::new_king(make_pos!(3, 3), Player::White, false),
            Piece::new_bishop(make_pos!(3, 2), Player::Black),
            Piece::new_bishop(make_pos!(3, 4), Player::Black),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 2, 2),
            make_move!(3, 3, 4, 4),
            make_move!(3, 3, 4, 2),
            make_move!(3, 3, 2, 4),
            make_move!(3, 3, 3, 2),
            make_move!(3, 3, 3, 4),
            make_move!(3, 3, 2, 3),
            make_move!(3, 3, 4, 3),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_king_blocked() {
        let board = make_board!(
            Piece::new_king(make_pos!(3, 3), Player::White, false),
            Piece::new_bishop(make_pos!(3, 2), Player::White),
            Piece::new_bishop(make_pos!(3, 4), Player::White),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(3, 3);
        let mut expected_moves = vec![
            make_move!(3, 3, 2, 2),
            make_move!(3, 3, 4, 4),
            make_move!(3, 3, 4, 2),
            make_move!(3, 3, 2, 4),
            make_move!(3, 3, 2, 3),
            make_move!(3, 3, 4, 3),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }

    #[test]
    fn test_is_player_checked() {
        let board = make_board!(
            Piece::new_king(make_pos!(3, 3), Player::White, false),
            Piece::new_bishop(make_pos!(2, 2), Player::Black),
            Piece::new_bishop(make_pos!(3, 4), Player::White),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        assert!(state.is_checked());
    }

    #[test]
    fn test_transform_state() {
        let board = make_board!(Piece::new_bishop(make_pos!(2, 2), Player::Black),);
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let new_move = make_move!(2, 2, 3, 3);
        let promotion = None;

        let new_state = GameState::transform_state(&state, new_move, promotion);

        assert!(new_state.board[2][2].is_none());
        assert!(matches!(new_state.board[3][3], Some(Piece::Bishop(_))));

        assert_eq!(new_state.current_player, get_opponent(state.current_player));

        assert!(new_state.en_passant_square.is_none());
    }

    #[test]
    fn test_transform_state_with_promotion() {
        let board = make_board!(Piece::new_bishop(make_pos!(2, 2), Player::Black),);
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let new_move = make_move!(2, 2, 3, 3);
        let promotion = Some(PromotionType::Knight);

        let new_state = GameState::transform_state(&state, new_move, promotion);

        // Sprawdź, czy pionek został przeniesiony
        assert!(new_state.board[2][2].is_none());
        assert!(matches!(new_state.board[3][3], Some(Piece::Knight(_))));

        // Sprawdź, czy aktualny gracz został zmieniony
        assert_eq!(new_state.current_player, get_opponent(state.current_player));

        // Sprawdź, czy en_passant_square jest None
        assert!(new_state.en_passant_square.is_none());
    }

    #[test]
    fn test_update_en_passant() {
        let board = make_board!(
            Piece::new_pawn(make_pos!(2, 4), Player::Black, false),
            Piece::new_pawn(make_pos!(4, 5), Player::White, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let new_move = make_move!(2, 4, 4, 4);
        let result = GameState::update_en_passant(&state, &new_move);
        assert_eq!(result, Some(Position::new(4, 4).unwrap()));
    }

    #[test]
    fn test_king_castles() {
        let board = make_board!(
            Piece::new_king(make_pos!(0, 4), Player::White, true),
            Piece::new_rook(make_pos!(0, 0), Player::White, true),
            Piece::new_rook(make_pos!(0, 7), Player::White, true),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(0, 4);
        let mut expected_moves = vec![
            make_move!(0, 4, 0, 3),
            make_move!(0, 4, 1, 3),
            make_move!(0, 4, 1, 4),
            make_move!(0, 4, 1, 5),
            make_move!(0, 4, 0, 5),
            make_move!(0, 4, 0, 2),
            make_move!(0, 4, 0, 6),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_king_cant_castle_king_moved() {
        let board = make_board!(
            Piece::new_king(make_pos!(0, 4), Player::White, false),
            Piece::new_rook(make_pos!(0, 0), Player::White, true),
            Piece::new_rook(make_pos!(0, 7), Player::White, true),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(0, 4);
        let mut expected_moves = vec![
            make_move!(0, 4, 0, 3),
            make_move!(0, 4, 1, 3),
            make_move!(0, 4, 1, 4),
            make_move!(0, 4, 1, 5),
            make_move!(0, 4, 0, 5),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_king_cant_castle_rook_moved() {
        let board = make_board!(
            Piece::new_king(make_pos!(0, 4), Player::White, true),
            Piece::new_rook(make_pos!(0, 0), Player::White, false),
            Piece::new_rook(make_pos!(0, 7), Player::White, true),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(0, 4);
        let mut expected_moves = vec![
            make_move!(0, 4, 0, 3),
            make_move!(0, 4, 1, 3),
            make_move!(0, 4, 1, 4),
            make_move!(0, 4, 1, 5),
            make_move!(0, 4, 0, 5),
            make_move!(0, 4, 0, 6),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_king_cant_castle_blocked() {
        let board = make_board!(
            Piece::new_king(make_pos!(0, 4), Player::White, true),
            Piece::new_rook(make_pos!(0, 0), Player::White, true),
            Piece::new_knight(make_pos!(0, 1), Player::White),
            Piece::new_knight(make_pos!(0, 6), Player::White),
            Piece::new_rook(make_pos!(0, 7), Player::White, true),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(0, 4);
        let mut expected_moves = vec![
            make_move!(0, 4, 0, 3),
            make_move!(0, 4, 1, 3),
            make_move!(0, 4, 1, 4),
            make_move!(0, 4, 1, 5),
            make_move!(0, 4, 0, 5),
        ];
        test_get_moves(state, pos, &mut expected_moves);
    }
    #[test]
    fn test_king_cant_castle_under_check() {
        let board = make_board!(
            Piece::new_king(make_pos!(0, 4), Player::White, true),
            Piece::new_rook(make_pos!(0, 0), Player::White, true),
            Piece::new_rook(make_pos!(0, 7), Player::White, true),
            Piece::new_rook(make_pos!(7, 4), Player::Black, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        let pos = make_pos!(0, 4);
        check_move_not_in_get_moves(&state, &pos, &make_move!(0, 4, 0, 2));
        check_move_not_in_get_moves(&state, &pos, &make_move!(0, 4, 0, 6));
    }
    #[test]
    fn test_is_promotion() {
        let board = make_board!(
            Piece::new_pawn(make_pos!(6, 4), Player::White, false),
            Piece::new_pawn(make_pos!(5, 5), Player::White, false),
            Piece::new_rook(make_pos!(6, 6), Player::White, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        assert!(state.is_promotion_move(make_move!(6, 4, 7, 4)));
        assert!(!state.is_promotion_move(make_move!(5, 5, 6, 5)));
        assert!(!state.is_promotion_move(make_move!(6, 6, 7, 6)));
    }
    #[test]
    fn test_is_promotion_black() {
        let board = make_board!(
            Piece::new_pawn(make_pos!(1, 4), Player::Black, false),
            Piece::new_pawn(make_pos!(2, 5), Player::Black, false),
            Piece::new_rook(make_pos!(1, 6), Player::Black, false),
        );
        let state = GameState::from_board(board, Player::Black, None).unwrap();
        assert!(state.is_promotion_move(make_move!(1, 4, 0, 4)));
        assert!(!state.is_promotion_move(make_move!(2, 5, 1, 5)));
        assert!(!state.is_promotion_move(make_move!(1, 6, 0, 6)));
    }
    #[test]
    fn test_running() {
        let board = make_board!(
            Piece::new_king(make_pos!(0, 4), Player::White, false),
            Piece::new_king(make_pos!(7, 4), Player::Black, false),
            Piece::new_rook(make_pos!(1, 2), Player::White, false),
            Piece::new_rook(make_pos!(6, 5), Player::Black, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        assert!(!state.is_finished());
    }
    #[test]
    fn test_draw() {
        let board = make_board!(
            Piece::new_king(make_pos!(0, 4), Player::White, false),
            Piece::new_king(make_pos!(2, 4), Player::Black, false),
            Piece::new_pawn(make_pos!(1, 4), Player::Black, false),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        // assert!(state.is_finished());
        assert!(state.get_winner().is_none());
    }
    #[test]
    fn test_checkmate() {
        let board = make_board!(
            Piece::new_king(make_pos!(0, 4), Player::White, false),
            Piece::new_king(make_pos!(2, 4), Player::Black, false),
            Piece::new_queen(make_pos!(1, 4), Player::Black),
        );
        let state = GameState::from_board(board, Player::White, None).unwrap();
        // assert!(state.is_finished());
        assert!(state.get_winner().is_some());
        assert!(state.get_winner().unwrap() == Player::Black);
    }
}
