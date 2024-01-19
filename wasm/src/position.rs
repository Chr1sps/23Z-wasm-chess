use wasm_bindgen::prelude::*;
/// A struct representing a position on a chess board. It maps the range 0..=7
/// to rows (1..=8) and columns (A-H).
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    row: u8,
    column: u8,
}

impl Position {
    /// Returns a tuple of form (row, column).
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
    /// Returns the row of a position.
    pub fn get_row(&self) -> u8 {
        self.row
    }
    /// Returns the column of a position.
    pub fn get_column(&self) -> u8 {
        self.column
    }
    /// Returns an array for easier handling and conversion in JS. The array
    /// consists of two u8 values of form: [row, column];
    pub fn as_js_tuple(&self) -> Box<[u8]> {
        Box::new([self.row, self.column])
    }
}
