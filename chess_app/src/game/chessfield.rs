pub use chessman::ChessPieceTrait;
pub use chessman::Position;

pub mod chessman;

// pub enum Option<TChessPieceTrait>  {
//     Some(T),
//     None,
// }

pub struct ChessField {
    status: Option<Box<dyn ChessPieceTrait>>,
    position: Position,
}

impl ChessField{
    pub fn new(new_position: Position) -> Self {
        Self {
            status: None,
            position: new_position,
        }
    }

    pub fn get_status(&self) -> &Option<Box<dyn ChessPieceTrait>> {
        &self.status
    }

    pub fn set_status(&mut self, status: Option<Box<dyn ChessPieceTrait>>){
        self.status = status;
    }
    pub fn get_position(&self) -> &Position{
        &self.position
    }
}