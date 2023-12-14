use crate::{ChessField, ChessMove, Player};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone)]
pub enum PromotionType {
    Queen,
    Rook,
    Bishop,
    Knight,
}

pub struct GameState {
    board: Vec<Vec<ChessField>>,
    current_player: Player,
}
impl GameState {
    pub fn generate_next_state(
        old_state: GameState,
        r#move: ChessMove,
        promotion: Option<PromotionType>,
    ) -> Option<Self> {
    }
    pub fn first_state() -> Self {}

    pub fn get_moves(&self) -> Vec<ChessMove> {
        let mut result = vec![];
        for column in self.board {
            for field in column {
                if let Some(piece) = field.get_status() {
                    result.append(&mut piece.get_moves(&self));
                }
            }
        }
        result
    }

    pub fn check_promotion_move(&self, r#move: ChessMove) -> bool {
        let (column, row) = r#move.get_current_position().to_tuple();
        let field = self.board[column as usize][row as usize];
        match field.get_status() {
            Some(piece) => true,
            None => false,
        }
    }

    pub fn is_finished(&self) -> bool {}

    pub fn get_winner(&self) -> Player {}
}
