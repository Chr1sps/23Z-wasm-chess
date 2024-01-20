use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Black,
    White,
}

#[wasm_bindgen]
pub fn get_opponent(player: Player) -> Player {
    match player {
        Player::White => Player::Black,
        Player::Black => Player::White,
    }
}
