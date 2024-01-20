use gamestate::GameState;
use itertools::Itertools;
pub use piece::Piece;
pub use position::Position;
pub use r#move::Move;
use wasm_bindgen::prelude::*;

use crate::piece::{PieceData, PieceType};
use crate::player::Player;

mod gamestate;
pub mod r#move;
pub mod piece;
pub mod player;
pub mod position;
mod utils;

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

#[wasm_bindgen]
pub struct JsPos {
    pub row: u8,
    pub col: u8,
}

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
    pub fn get_moves(&self, row: u8, column: u8) -> Vec<JsPos> {
        let pos = Position::new(row, column).unwrap();
        self.state
            .get_moves(pos)
            .into_iter()
            .map(move |x| {
                let pos = x.get_end_position();
                JsPos {
                    row: pos.get_row(),
                    col: pos.get_column(),
                }
            })
            .collect_vec()
    }
    /// Returns true if the game has finished.
    pub fn is_finished(&self) -> bool {
        self.state.is_finished()
    }
    /// Returns true if the move would result in a promotion of a pawn.
    pub fn is_promotion_move(
        &self,
        from_row: u8,
        from_column: u8,
        to_row: u8,
        to_column: u8,
    ) -> bool {
        let from = Position::new(from_row, from_column).unwrap();
        let to = Position::new(to_row, to_column).unwrap();
        self.state.is_promotion_move(Move::new(from, to))
    }
    /// Tries to make a move; returns Ok(()) if the move was successful,
    /// Err(String) otherwise.
    pub fn make_move(
        &mut self,
        from_row: u8,
        from_column: u8,
        to_row: u8,
        to_column: u8,
        promotion_type: Option<PromotionType>,
    ) -> Result<(), String> {
        let from = Position::new(from_row, from_column).unwrap();
        let to = Position::new(to_row, to_column).unwrap();
        let chess_move = Move::new(from, to);
        let possible_moves = self
            .state
            .get_moves(chess_move.get_current_position())
            .into_iter()
            .map(move |x| x.get_end_position())
            .collect_vec();
        if possible_moves.contains(&chess_move.get_end_position()) {
            self.state = GameState::transform_state(&self.state, chess_move, promotion_type);
            Ok(())
        } else {
            Err("Invalid move".to_string())
        }
    }

    /// Returns Some(Player) if a game has resulted in a win for a given player,
    /// None otherwise.
    pub fn get_winner(&self) -> Option<Player> {
        self.state.get_winner()
    }

    /// Returns a PieceData struct containing info about the player and the
    /// type of the piece used.
    pub fn get_piece_data(&self, row: u8, column: u8) -> Option<PieceData> {
        self.state
            .get_piece(Position::new(row, column).unwrap())
            .map(|piece| piece.get_data())
    }
}
