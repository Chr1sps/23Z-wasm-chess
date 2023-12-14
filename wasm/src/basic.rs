use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PlayerKind {
    Black,
    White,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Position {
    row: u8,
    column: u8,
}

impl Position {
    pub fn new(row: u8, column: u8) -> Option<Self> {
        let range = (1..=8 as u8);
        if (range.contains(&row) && range.contains(&column)) {
            Some(Self { row, column })
        } else {
            None
        }
    }

    pub fn get_row(&self) -> u8 {
        self.row
    }
    pub fn get_column(&self) -> u8 {
        self.column
    }

    pub fn to_tuple(&self) -> (u8, u8) {
        (self.column, self.row)
    }
}
