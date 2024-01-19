use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn get_enemy(&self) -> Self {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}
