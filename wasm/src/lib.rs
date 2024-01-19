use gamestate::GameState;
use itertools::Itertools;
pub use piece::Piece;
pub use position::Position;
pub use r#move::Move;
use wasm_bindgen::prelude::*;

use crate::piece::PieceType;
use crate::player::Player;

mod gamestate;
pub mod r#move;
pub mod piece;
pub mod player;
pub mod position;
mod utils;

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
    state: GameState,
}

// pub fn r#move(&self, position: Position) -> Result<Vec<Move>, String> {
//     let (row, column) = position.to_tuple();
//     let mut filtered_results: Vec<Move> = vec![];
//     let chessfield: &Field = self.get_field(row, column);
//     match chessfield.get_piece() {
//         None => Err("You must select field with chesspiece".to_string()),
//         Some(chessman) => {
//             let results: Vec<Move> = chessman.get_moves(&self);
//             for result in results {
//                 let end_pos = result.get_end_position();
//                 let possible_field: &Field =
//                     self.get_field(end_pos.get_row(), end_pos.get_column());
//                 if (possible_field.get_piece().is_some()
//                     && chessman.get_player()
//                         == possible_field.get_piece().as_ref().unwrap().get_player())
//                 .not()
//                 {
//                     filtered_results.push(result);
//                 }
//             }
//             println!("{:#?}", filtered_results);
//             println!("{}", filtered_results.len());
//             Ok(filtered_results)
//         }
//     }
// }

// pub fn make_pawn_move(&self, start_position: Position) -> Result<Vec<Move>, String> {
//     let mut results: Vec<Move> = vec![];

//     let (row, column) = start_position.to_tuple();
//     let right_column = column + 1;
//     let left_column = row - 1;
//     let row_shift: i32 = if self.get_current_player() == Player::White {
//         1
//     } else {
//         -1
//     };

//     let chessfield: &Field = self.get_field(row, column);

//     if self
//         .get_field((row as i32 + row_shift) as u8, column)
//         .get_piece()
//         .is_none()
//     {
//         let chessmove = chessfield
//             .get_piece()
//             .as_ref()
//             .unwrap()
//             .create_move(column, (row as i32 + row_shift) as u8);
//         results.push(chessmove.unwrap());
//     }

//     if chessfield.get_piece().as_ref().unwrap().is_first_move()
//         && self
//             .get_field((row as i32 + row_shift) as u8, column)
//             .get_piece()
//             .is_none()
//         && self
//             .get_field((row as i32 + 2 * row_shift) as u8, column)
//             .get_piece()
//             .is_none()
//     {
//         let chessmove = chessfield
//             .get_piece()
//             .as_ref()
//             .unwrap()
//             .create_move(column, (row as i32 + 2 * row_shift) as u8);
//         results.push(chessmove.unwrap());
//     }

//     if (0..8 as u8).contains(&right_column) {
//         let right_diagonal = self
//             .get_field((row as i32 + row_shift) as u8, right_column)
//             .get_piece();
//         if right_diagonal.is_some()
//             && right_diagonal.as_ref().unwrap().get_player() != self.get_current_player()
//         {
//             let chessmove = chessfield
//                 .get_piece()
//                 .as_ref()
//                 .unwrap()
//                 .create_move(right_column, (row as i32 + row_shift) as u8);
//             results.push(chessmove.unwrap());
//         }
//         // is_en_passantable
//     }

//     if (0..8 as u8).contains(&left_column) {
//         let left_diagonal = self
//             .get_field((row as i32 + row_shift) as u8, left_column)
//             .get_piece();
//         if left_diagonal.is_some()
//             && left_diagonal.as_ref().unwrap().get_player() != self.get_current_player()
//         {
//             let chessmove = chessfield
//                 .get_piece()
//                 .as_ref()
//                 .unwrap()
//                 .create_move(left_column, (row as i32 + row_shift) as u8);
//             results.push(chessmove.unwrap());
//         }
//         // is_en_passantable
//     }
//     println!("{:#?}", results);
//     println!("{}", results.len());
//     Ok(results)
// }

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        utils::set_panic_hook();
        Self {
            state: GameState::init(),
        }
    }
    /// Returns a list of possible positions that a piece on a given square can
    /// get to within a move.
    pub fn get_moves(&self, pos: Position) -> Vec<Position> {
        self.state
            .get_all_moves()
            .into_iter()
            .map(move |x| x.get_end_position())
            .collect_vec()
    }
    /// Returns true if the game has finished.
    pub fn is_finished(&self) -> bool {
        self.state.is_finished()
    }
    /// Returns true if the move would result in a promotion of a pawn.
    pub fn is_promotion_move(&self, chess_move: Move) -> bool {
        // self.state.is_promotion_move(chess_move)
        false
    }
    /// Tries to make a move; returns Ok(()) if the move was successful,
    /// Err(String) otherwise.
    pub fn make_move(
        &mut self,
        chess_move: Move,
        promotion_type: Option<PromotionType>,
    ) -> Result<(), String> {
        Ok(())
    }

    /// Returns Some(Player) if a game has resulted in a win for a given player,
    /// None otherwise.
    pub fn get_winner(&self) -> Option<Player> {
        self.state.get_winner()
    }

    pub fn get_piece_type(&self, position: Position) -> Option<PieceType> {
        self.state.get_piece(position).map(|piece| piece.get_type())
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::piece::{Bishop, King, Knight, Pawn, Queen, Rook};

//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_change_round() {
//         let mut game = Game::new();
//         game.change_round();
//         assert_eq!(game.get_current_player(), Player::Black);
//         assert_eq!(game.get_other_player(), Player::White);
//     }

//     #[test]
//     fn test_rook_movement() {
//         let mut game = Game::new();
//         game.set_field(
//             (4, 4),
//             Rook::new(Position::new(4, 4).unwrap(), Player::White),
//         );
//         let result = game.r#move(Position::new(4, 4).unwrap());
//         assert_eq!(result.unwrap().len(), 14);
//     }

//     #[test]
//     fn test_queen_movement() {
//         let mut game = Game::new();
//         game.set_field(
//             (4, 4),
//             Queen::new(Position::new(4, 4).unwrap(), Player::White),
//         );
//         let result = game.r#move(Position::new(4, 4).unwrap());
//         assert_eq!(result.unwrap().len(), 27);
//     }

//     #[test]
//     fn test_king_movement() {
//         let mut game = Game::new();
//         game.set_field(
//             (4, 4),
//             King::new(Position::new(4, 4).unwrap(), Player::White),
//         );
//         let result = game.r#move(Position::new(4, 4).unwrap());
//         assert_eq!(result.unwrap().len(), 8);
//     }

//     #[test]
//     fn test_bishop_movement() {
//         let mut game = Game::new();
//         game.set_field(
//             (4, 4),
//             Bishop::new(Position::new(4, 4).unwrap(), Player::White),
//         );
//         let result = game.r#move(Position::new(4, 4).unwrap());
//         assert_eq!(result.unwrap().len(), 13);
//     }

//     #[test]
//     fn test_knight_movement() {
//         let mut game = Game::new();
//         game.set_field(
//             (4, 4),
//             Knight::new(Position::new(4, 4).unwrap(), Player::White),
//         );
//         let result = game.r#move(Position::new(4, 4).unwrap());
//         assert_eq!(result.unwrap().len(), 8);
//     }

//     #[test]
//     fn test_pawn_movement() {
//         let mut game = Game::new();
//         game.set_field(
//             (4, 4),
//             Pawn::new(Position::new(4, 4).unwrap(), Player::White),
//         );
//         let result = game.make_pawn_move(Position::new(4, 4).unwrap());
//         assert_eq!(result.unwrap().len(), 2);
//     }
// }
