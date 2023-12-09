pub use chessman::Chesspiece;
pub use chessman::Position;

pub mod chessman;

// pub enum Option<T: Chesspiece>  {
//     Some(T),
//     None,
// }

pub struct ChessField {
    status: Option<Box<dyn Chesspiece>>,
    position: Position,
}

impl ChessField{
    pub fn new(new_position: Position) -> Self {
        Self {
            status: None,
            position: new_position,
        }
    }
}