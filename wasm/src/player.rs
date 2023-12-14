use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Black,
    White,
}
