pub use chessman::ChessMove;
pub use chessman::ChessPieceTrait;
pub use field::ChessField;
pub use position::PlayerKind;
pub use position::Position;
use std::mem;
use std::ops::Not;
use std::vec;
use wasm_bindgen::prelude::*;

use crate::player::Player;

pub mod chessman;
pub mod field;
// mod gamestate;
pub mod player;
pub mod position;

#[derive(Clone, Copy)]
pub enum Status {
    Check,
    Checkmate,
    Normal,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone)]
pub enum PromotionType {
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[wasm_bindgen]
pub struct Game {
    current_player: PlayerKind,
    other_player: PlayerKind,
    status: Status,
    chessboard: Vec<Vec<ChessField>>,
}

impl Game {
    pub fn get_game_status(&self) -> &Status {
        &self.status
    }

    pub fn set_game_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    pub fn get_current_player(&self) -> PlayerKind {
        self.current_player
    }

    pub fn get_other_player(&self) -> PlayerKind {
        self.other_player
    }

    pub fn change_round(&mut self) {
        mem::swap(&mut self.current_player, &mut self.other_player)
    }

    fn init_board() -> Vec<Vec<ChessField>> {
        let mut chessboard: Vec<Vec<ChessField>> = Vec::with_capacity(8);

        for i in 0..8 {
            let mut row: Vec<ChessField> = Vec::with_capacity(8);
            for j in 0..8 {
                row.push(ChessField::new(Position::new(i, j).unwrap()));
            }
            chessboard.push(row);
        }
        chessboard
    }

    pub fn r#move(&self, position: Position) -> Result<Vec<ChessMove>, String> {
        let (row, column) = position.to_tuple();
        let mut filtered_results: Vec<ChessMove> = vec![];
        let chessfield: &ChessField = self.get_field(row, column);
        match chessfield.get_status() {
            None => Err("You must select field with chesspiece".to_string()),
            Some(chessman) => {
                let results: Vec<ChessMove> = chessman.get_moves();
                for result in results {
                    let end_pos = result.get_end_position();
                    let possible_field: &ChessField =
                        self.get_field(end_pos.get_row(), end_pos.get_column());
                    if (possible_field.get_status().is_some()
                        && chessman.get_player()
                            == possible_field.get_status().as_ref().unwrap().get_player())
                    .not()
                    {
                        filtered_results.push(result);
                    }
                }
                println!("{:#?}", filtered_results);
                println!("{}", filtered_results.len());
                Ok(filtered_results)
            }
        }
    }

    pub fn make_pawn_move(&self, start_position: Position) -> Result<Vec<ChessMove>, String> {
        let mut results: Vec<ChessMove> = vec![];

        let (row, column) = start_position.to_tuple();
        let right_column = column + 1;
        let left_column = row - 1;
        let row_shift: i32 = if self.get_current_player() == PlayerKind::White {
            1
        } else {
            -1
        };

        let chessfield: &ChessField = self.get_field(row, column);

        if self
            .get_field((row as i32 + row_shift) as u8, column)
            .get_status()
            .is_none()
        {
            let chessmove = chessfield
                .get_status()
                .as_ref()
                .unwrap()
                .create_move(column, (row as i32 + row_shift) as u8);
            results.push(chessmove.unwrap());
        }

        if chessfield.get_status().as_ref().unwrap().is_first_move()
            && self
                .get_field((row as i32 + row_shift) as u8, column)
                .get_status()
                .is_none()
            && self
                .get_field((row as i32 + 2 * row_shift) as u8, column)
                .get_status()
                .is_none()
        {
            let chessmove = chessfield
                .get_status()
                .as_ref()
                .unwrap()
                .create_move(column, (row as i32 + 2 * row_shift) as u8);
            results.push(chessmove.unwrap());
        }

        if (0..8 as u8).contains(&right_column) {
            let right_diagonal = self
                .get_field((row as i32 + row_shift) as u8, right_column)
                .get_status();
            if right_diagonal.is_some()
                && right_diagonal.as_ref().unwrap().get_player() != self.get_current_player()
            {
                let chessmove = chessfield
                    .get_status()
                    .as_ref()
                    .unwrap()
                    .create_move(right_column, (row as i32 + row_shift) as u8);
                results.push(chessmove.unwrap());
            }
            // is_en_passantable
        }

        if (0..8 as u8).contains(&left_column) {
            let left_diagonal = self
                .get_field((row as i32 + row_shift) as u8, left_column)
                .get_status();
            if left_diagonal.is_some()
                && left_diagonal.as_ref().unwrap().get_player() != self.get_current_player()
            {
                let chessmove = chessfield
                    .get_status()
                    .as_ref()
                    .unwrap()
                    .create_move(left_column, (row as i32 + row_shift) as u8);
                results.push(chessmove.unwrap());
            }
            // is_en_passantable
        }
        println!("{:#?}", results);
        println!("{}", results.len());
        Ok(results)
    }

    pub fn set_field<T: ChessPieceTrait + 'static>(&mut self, position: (i32, i32), chesspiece: T) {
        let (row, column) = position;
        let row_i = row as usize;
        let column_i = column as usize;
        if let Some(field) = self
            .chessboard
            .get_mut(row_i)
            .and_then(|row| row.get_mut(column_i))
        {
            field.set_status(Some(Box::new(chesspiece)));
        }
    }

    pub fn get_field(&self, row: u8, column: u8) -> &ChessField {
        let idx_row = row as usize;
        let idx_column = column as usize;
        &self.chessboard[idx_row][idx_column]
    }
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Self {
            current_player: PlayerKind::White,
            other_player: PlayerKind::Black,
            status: Status::Normal,
            chessboard: Game::init_board(),
        }
    }
    /// Returns a list of possible positions that a piece on a given square can
    /// get to within a move.
    pub fn get_moves(&self, pos: Position) -> Vec<Position> {
        vec![]
    }
    /// Returns true if the game has finished.
    pub fn is_finished(&self) -> bool {
        false
    }
    /// Returns true if the move would result in a promotion of a pawn.
    pub fn is_promotion_move(&self, chess_move: ChessMove) -> bool {
        false
    }
    /// Tries to make a move; returns Ok(()) if the move was successful,
    /// Err(String) otherwise.
    pub fn make_move(
        &mut self,
        chess_move: ChessMove,
        promotion_type: Option<PromotionType>,
    ) -> Result<(), String> {
        Ok(())
    }

    /// Returns Some(Player) if a game has resulted in a win for a given player,
    /// None otherwise.
    pub fn get_winner(&self) -> Option<Player> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::chessman::{Bishop, King, Knight, Pawn, Queen, Rook};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_change_round() {
        let mut game = Game::new();
        game.change_round();
        assert_eq!(game.get_current_player(), PlayerKind::Black);
        assert_eq!(game.get_other_player(), PlayerKind::White);
    }

    #[test]
    fn test_rook_movement() {
        let mut game = Game::new();
        game.set_field(
            (4, 4),
            Rook::new(Position::new(4, 4).unwrap(), PlayerKind::White),
        );
        let result = game.r#move(Position::new(4, 4).unwrap());
        assert_eq!(result.unwrap().len(), 14);
    }

    #[test]
    fn test_queen_movement() {
        let mut game = Game::new();
        game.set_field(
            (4, 4),
            Queen::new(Position::new(4, 4).unwrap(), PlayerKind::White),
        );
        let result = game.r#move(Position::new(4, 4).unwrap());
        assert_eq!(result.unwrap().len(), 27);
    }

    #[test]
    fn test_king_movement() {
        let mut game = Game::new();
        game.set_field(
            (4, 4),
            King::new(Position::new(4, 4).unwrap(), PlayerKind::White),
        );
        let result = game.r#move(Position::new(4, 4).unwrap());
        assert_eq!(result.unwrap().len(), 8);
    }

    #[test]
    fn test_bishop_movement() {
        let mut game = Game::new();
        game.set_field(
            (4, 4),
            Bishop::new(Position::new(4, 4).unwrap(), PlayerKind::White),
        );
        let result = game.r#move(Position::new(4, 4).unwrap());
        assert_eq!(result.unwrap().len(), 13);
    }

    #[test]
    fn test_knight_movement() {
        let mut game = Game::new();
        game.set_field(
            (4, 4),
            Knight::new(Position::new(4, 4).unwrap(), PlayerKind::White),
        );
        let result = game.r#move(Position::new(4, 4).unwrap());
        assert_eq!(result.unwrap().len(), 8);
    }

    #[test]
    fn test_pawn_movement() {
        let mut game = Game::new();
        game.set_field(
            (4, 4),
            Pawn::new(Position::new(4, 4).unwrap(), PlayerKind::White),
        );
        let result = game.make_pawn_move(Position::new(4, 4).unwrap());
        assert_eq!(result.unwrap().len(), 2);
    }
}
