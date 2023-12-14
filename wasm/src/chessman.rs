pub use crate::position::PlayerKind;
pub use crate::position::Position;
use core::any::Any;
use itertools::iproduct;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct ChessMove {
    current_position: Position,
    end_position: Position,
}

impl ChessMove {
    pub fn get_end_position(&self) -> Position {
        self.end_position
    }

    pub fn get_current_position(&self) -> Position {
        self.current_position
    }
}

pub struct ChessPiece {
    position: Position,
    player: PlayerKind,
}

impl ChessPiece {
    pub fn new(new_position: Position, new_player: PlayerKind) -> ChessPiece {
        ChessPiece {
            position: new_position,
            player: new_player,
        }
    }
}

pub trait ChessPieceTrait: Any {
    fn is_first_move(&self) -> bool {
        false
    }

    fn get_moves_lines(&self, directions: Vec<(i32, i32)>) -> Vec<ChessMove> {
        let mut results: Vec<ChessMove> = vec![];
        for (dir_column, dir_row) in directions {
            for distance in 1..8 {
                let new_column =
                    ((self.get_position().get_column() as i32) + distance * dir_column) as u8;
                let new_row = ((self.get_position().get_row() as i32) + distance * dir_row) as u8;
                match self.create_move(new_column, new_row) {
                    None => continue,
                    Some(chessmove) => results.push(chessmove),
                }
            }
        }
        results
    }

    fn get_moves_shifts(&self, shifts: Vec<(i32, i32)>) -> Vec<ChessMove> {
        let mut result: Vec<ChessMove> = vec![];
        for (column_shift, row_shift) in shifts {
            let new_column = (self.get_position().get_column() as i32 + column_shift) as u8;
            let new_row = (self.get_position().get_row() as i32 + row_shift) as u8;
            if let Some(result_move) = self.create_move(new_column, new_row) {
                result.push(result_move);
            }
        }
        result
    }

    fn create_move(&self, new_column: u8, new_row: u8) -> Option<ChessMove> {
        if let Some(end_position) = Position::new(new_row, new_column) {
            Some(ChessMove {
                current_position: self.get_position(),
                end_position,
            })
        } else {
            None
        }
    }

    fn calculate_params(&self) -> Vec<(i32, i32)> {
        let a = vec![1, 0, -1];
        let b = vec![1, 0, -1];
        let mut params: Vec<(i32, i32)> = iproduct!(a, b).collect();
        params.retain(|&x| x != (0, 0));
        params
    }

    fn get_moves(&self) -> Vec<ChessMove>;

    fn get_player(&self) -> PlayerKind;

    fn get_position(&self) -> Position;
}

pub struct Pawn {
    chessman: ChessPiece,
    pub first_move: bool,
    #[allow(dead_code)]
    is_en_passantable: bool,
}

impl Pawn {
    pub fn new(position: Position, player: PlayerKind) -> Pawn {
        Pawn {
            chessman: ChessPiece::new(position, player),
            first_move: true,
            is_en_passantable: false,
        }
    }
}

impl ChessPieceTrait for Pawn {
    fn get_moves(&self) -> Vec<ChessMove> {
        let result: Vec<ChessMove> = vec![];
        result
    }

    fn is_first_move(&self) -> bool {
        self.first_move
    }

    fn get_player(&self) -> PlayerKind {
        self.chessman.player
    }

    fn get_position(&self) -> Position {
        self.chessman.position
    }
}

pub struct Rook {
    chessman: ChessPiece,
}

impl Rook {
    pub fn new(position: Position, player: PlayerKind) -> Rook {
        Rook {
            chessman: ChessPiece::new(position, player),
        }
    }
}

impl ChessPieceTrait for Rook {
    fn get_moves(&self) -> Vec<ChessMove> {
        let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        self.get_moves_lines(directions)
    }

    fn get_player(&self) -> PlayerKind {
        self.chessman.player
    }

    fn get_position(&self) -> Position {
        self.chessman.position
    }
}

pub struct King {
    chessman: ChessPiece,
}

impl King {
    pub fn new(position: Position, player: PlayerKind) -> King {
        King {
            chessman: ChessPiece::new(position, player),
        }
    }
}

impl ChessPieceTrait for King {
    fn get_moves(&self) -> Vec<ChessMove> {
        let possible_shifts: Vec<(i32, i32)> = self.calculate_params();
        self.get_moves_shifts(possible_shifts)
    }

    fn get_player(&self) -> PlayerKind {
        self.chessman.player
    }

    fn get_position(&self) -> Position {
        self.chessman.position
    }
}

pub struct Queen {
    chessman: ChessPiece,
}

impl Queen {
    pub fn new(position: Position, player: PlayerKind) -> Queen {
        Queen {
            chessman: ChessPiece::new(position, player),
        }
    }
}

impl ChessPieceTrait for Queen {
    fn get_moves(&self) -> Vec<ChessMove> {
        let directions: Vec<(i32, i32)> = self.calculate_params();
        return self.get_moves_lines(directions);
    }

    fn get_player(&self) -> PlayerKind {
        self.chessman.player
    }

    fn get_position(&self) -> Position {
        self.chessman.position
    }
}

pub struct Bishop {
    chessman: ChessPiece,
}
impl Bishop {
    pub fn new(position: Position, player: PlayerKind) -> Bishop {
        Bishop {
            chessman: ChessPiece::new(position, player),
        }
    }
}

impl ChessPieceTrait for Bishop {
    fn get_moves(&self) -> Vec<ChessMove> {
        let a = vec![-1, 1];
        let b = vec![-1, 1];
        return self.get_moves_lines(iproduct!(a, b).collect());
    }

    fn get_player(&self) -> PlayerKind {
        self.chessman.player
    }

    fn get_position(&self) -> Position {
        self.chessman.position
    }
}

pub struct Knight {
    chessman: ChessPiece,
}

impl Knight {
    pub fn new(position: Position, player: PlayerKind) -> Knight {
        Knight {
            chessman: ChessPiece::new(position, player),
        }
    }
}

impl ChessPieceTrait for Knight {
    fn get_moves(&self) -> Vec<ChessMove> {
        let possible_shifts: Vec<(i32, i32)> = self.calculate_params();
        return self.get_moves_shifts(possible_shifts);
    }

    fn calculate_params(&self) -> Vec<(i32, i32)> {
        let a = vec![-2, 2];
        let b = vec![-1, 1];
        let mut possible_shifts: Vec<(i32, i32)> = iproduct!(a.clone(), b.clone()).collect();
        possible_shifts.extend(iproduct!(b, a).collect::<Vec<(i32, i32)>>());
        possible_shifts
    }

    fn get_player(&self) -> PlayerKind {
        self.chessman.player
    }

    fn get_position(&self) -> Position {
        self.chessman.position
    }
}
