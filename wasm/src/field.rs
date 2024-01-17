pub use crate::chessman::ChessPieceTrait;
pub use crate::chessman::Position;

pub struct ChessField {
    status: Option<Box<dyn ChessPieceTrait>>,
    position: Position,
}

impl ChessField {
    pub fn new(new_position: Position) -> Self {
        Self {
            status: None,
            position: new_position,
        }
    }

    pub fn get_piece(&self) -> &Option<Box<dyn ChessPieceTrait>> {
        &self.status
    }

    pub fn set_piece(&mut self, status: Option<Box<dyn ChessPieceTrait>>) {
        self.status = status;
    }
    pub fn get_position(&self) -> &Position {
        &self.position
    }
}
