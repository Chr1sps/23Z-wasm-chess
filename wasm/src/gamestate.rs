use crate::{chessman::Piece, ChessField, Move, Player, Position, PromotionType};
use wasm_bindgen::prelude::*;

pub struct GameState {
    board: Vec<Vec<ChessField>>,
    current_player: Player,
}
impl GameState {
    pub fn generate_next_state(
        old_state: GameState,
        r#move: Move,
        promotion: Option<PromotionType>,
    ) -> Option<Self> {
        None
    }
    pub fn first_state() -> Self {
        unimplemented!()
    }

    pub fn get_moves(&self) -> Vec<Move> {
        let mut result = vec![];
        for column in self.board {
            for field in column {
                if let Some(piece) = field.get_piece() {
                    result.append(&mut piece.get_moves(&self));
                }
            }
        }
        result
    }

    pub fn check_promotion_move(&self, r#move: Move) -> bool {
        let (column, row) = r#move.get_current_position().to_tuple();
        let field = self.board[column as usize][row as usize];
        match field.get_piece() {
            Some(piece) => true,
            None => false,
        }
    }

    pub fn is_finished(&self) -> bool {
        unimplemented!()
    }

    pub fn get_winner(&self) -> Option<Player> {
        unimplemented!()
    }
    pub fn get_piece(&self, position: Position) -> Option<&Piece> {
        unimplemented!()
    }
}
