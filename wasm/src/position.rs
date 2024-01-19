use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    row: u8,
    column: u8,
}

impl Position {
    pub fn as_tuple(&self) -> (u8, u8) {
        (self.row, self.column)
    }
}

#[wasm_bindgen]
impl Position {
    /// Creates a new position if both the row and the column are within a
    /// range 0..=7, otherwise returns no value.
    pub fn new(row: u8, column: u8) -> Option<Position> {
        let range = 0..=7 as u8;
        if range.contains(&row) && range.contains(&column) {
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
    /// Returns an array for easier handling and conversion in JS.
    pub fn as_js_tuple(&self) -> Box<[u8]> {
        Box::new([self.row, self.column])
    }
}
