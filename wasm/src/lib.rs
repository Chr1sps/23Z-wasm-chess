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
            .get_moves(pos)
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
        self.state.is_promotion_move(chess_move)
    }
    /// Tries to make a move; returns Ok(()) if the move was successful,
    /// Err(String) otherwise.
    pub fn make_move(
        &mut self,
        chess_move: Move,
        promotion_type: Option<PromotionType>,
    ) -> Result<(), String> {
        let possible_moves = self.get_moves(chess_move.get_current_position());
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

    /// Return a PieceType enum variant indicating the currently residing
    /// piece or null if there is none.
    pub fn get_piece_type(&self, position: Position) -> Option<PieceType> {
        self.state.get_piece(position).map(|piece| piece.get_type())
    }
}

