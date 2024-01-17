use crate::gamestate::GameState;
pub use crate::player::Player;
pub use crate::position::Position;
pub use crate::r#move::Move;
use core::any::Any;
use itertools::iproduct;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
pub struct ChessPiece {
    position: Position,
    player: Player,
}

impl ChessPiece {
    pub fn new(new_position: Position, new_player: Player) -> ChessPiece {
        ChessPiece {
            position: new_position,
            player: new_player,
        }
    }
}

pub trait ChessPieceTrait: Any {
    fn get_moves_lines(&self, directions: Vec<(i32, i32)>) -> Vec<Move> {
        let mut results: Vec<Move> = vec![];
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

    fn get_moves_shifts(&self, shifts: Vec<(i32, i32)>) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];
        for (column_shift, row_shift) in shifts {
            let new_column = (self.get_position().get_column() as i32 + column_shift) as u8;
            let new_row = (self.get_position().get_row() as i32 + row_shift) as u8;
            if let Some(result_move) = self.create_move(new_column, new_row) {
                result.push(result_move);
            }
        }
        result
    }

    fn create_move(&self, new_column: u8, new_row: u8) -> Option<Move> {
        if let Some(end_position) = Position::new(new_row, new_column) {
            Some(Move::new(self.get_position(), end_position))
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

    fn get_moves(&self, state: &GameState) -> Vec<Move>;

    fn get_player(&self) -> Player;

    fn get_position(&self) -> Position;
}

pub struct Pawn {
    chessman: ChessPiece,
    pub first_move: bool,
    #[allow(dead_code)]
    is_en_passantable: bool,
}

impl Pawn {
    fn is_first_move(&self) -> bool {
        self.first_move
    }

    pub fn new(position: Position, player: Player) -> Pawn {
        Pawn {
            chessman: ChessPiece::new(position, player),
            first_move: true,
            is_en_passantable: false,
        }
    }
}

impl ChessPieceTrait for Pawn {
    fn get_moves(&self) -> Vec<Move> {
        let result: Vec<Move> = vec![];
        result
    }

    fn get_player(&self) -> Player {
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
    pub fn new(position: Position, player: Player) -> Rook {
        Rook {
            chessman: ChessPiece::new(position, player),
        }
    }
}

impl ChessPieceTrait for Rook {
    fn get_moves(&self) -> Vec<Move> {
        let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        self.get_moves_lines(directions)
    }

    fn get_player(&self) -> Player {
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
    pub fn new(position: Position, player: Player) -> King {
        King {
            chessman: ChessPiece::new(position, player),
        }
    }
}

impl ChessPieceTrait for King {
    fn get_moves(&self) -> Vec<Move> {
        let possible_shifts: Vec<(i32, i32)> = self.calculate_params();
        self.get_moves_shifts(possible_shifts)
    }

    fn get_player(&self) -> Player {
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
    pub fn new(position: Position, player: Player) -> Queen {
        Queen {
            chessman: ChessPiece::new(position, player),
        }
    }
}

impl ChessPieceTrait for Queen {
    fn get_moves(&self) -> Vec<Move> {
        let directions: Vec<(i32, i32)> = self.calculate_params();
        return self.get_moves_lines(directions);
    }

    fn get_player(&self) -> Player {
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
    pub fn new(position: Position, player: Player) -> Bishop {
        Bishop {
            chessman: ChessPiece::new(position, player),
        }
    }
}

impl ChessPieceTrait for Bishop {
    fn get_moves(&self) -> Vec<Move> {
        let a = vec![-1, 1];
        let b = vec![-1, 1];
        return self.get_moves_lines(iproduct!(a, b).collect());
    }

    fn get_player(&self) -> Player {
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
    pub fn new(position: Position, player: Player) -> Knight {
        Knight {
            chessman: ChessPiece::new(position, player),
        }
    }
}

impl ChessPieceTrait for Knight {
    fn get_moves(&self) -> Vec<Move> {
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

    fn get_player(&self) -> Player {
        self.chessman.player
    }

    fn get_position(&self) -> Position {
        self.chessman.position
    }
}
