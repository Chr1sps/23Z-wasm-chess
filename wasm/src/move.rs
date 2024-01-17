use wasm_bindgen::prelude::*;

use crate::Position;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Move {
    current_position: Position,
    end_position: Position,
}

impl Move {
    pub fn get_end_position(&self) -> Position {
        self.end_position
    }

    pub fn get_current_position(&self) -> Position {
        self.current_position
    }
}

#[wasm_bindgen]
impl Move {
    pub fn new(from: Position, to: Position) -> Move {
        Self {
            current_position: from,
            end_position: to,
        }
    }
}