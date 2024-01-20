use crate::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    current_position: Position,
    end_position: Position,
}

impl Move {
    pub fn new(from: Position, to: Position) -> Move {
        Self {
            current_position: from,
            end_position: to,
        }
    }
    pub fn get_end_position(&self) -> Position {
        self.end_position
    }

    pub fn get_current_position(&self) -> Position {
        self.current_position
    }
}
