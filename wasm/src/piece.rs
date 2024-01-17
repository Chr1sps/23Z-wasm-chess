use crate::gamestate::GameState;
pub use crate::player::Player;
pub use crate::position::Position;
pub use crate::r#move::Move;
use itertools::{iproduct, Itertools};
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

pub struct PieceData {
    position: Position,
    player: Player,
}

impl PieceData {
    pub fn new(new_position: Position, new_player: Player) -> PieceData {
        PieceData {
            position: new_position,
            player: new_player,
        }
    }
}
pub enum Piece {
    Pawn(PieceData, bool, bool),
    Knight(PieceData),
    Bishop(PieceData),
    Rook(PieceData, bool),
    Queen(PieceData),
    King(PieceData, bool),
}

impl Piece {
    /// Creates a new pawn.
    pub fn new_pawn(
        position: Position,
        player: Player,
        first_move: bool,
        is_en_passantable: bool,
    ) -> Self {
        Self::Pawn(
            PieceData::new(position, player),
            first_move,
            is_en_passantable,
        )
    }
    /// Creates a new knight.
    pub fn new_knight(position: Position, player: Player) -> Self {
        Self::Knight(PieceData::new(position, player))
    }
    /// Creates a new bishop.
    pub fn new_bishop(position: Position, player: Player) -> Self {
        Self::Bishop(PieceData::new(position, player))
    }
    /// Creates a new rook.
    pub fn new_rook(position: Position, player: Player, can_castle: bool) -> Self {
        Self::Rook(PieceData::new(position, player), can_castle)
    }
    /// Creates a new queen.
    pub fn new_queen(position: Position, player: Player) -> Self {
        Self::Queen(PieceData::new(position, player))
    }
    /// Creates a new king.
    pub fn new_king(position: Position, player: Player, can_castle: bool) -> Self {
        Self::Rook(PieceData::new(position, player), can_castle)
    }

    /// Returns the piece's player.
    pub fn get_player(&self) -> Player {
        match self {
            Self::Pawn(data, _, _)
            | Self::Knight(data)
            | Self::Bishop(data)
            | Self::Rook(data, _)
            | Self::Queen(data)
            | Self::King(data, _) => data.player,
        }
    }

    /// Returns the piece's position.
    pub fn get_position(&self) -> Position {
        match self {
            Self::Pawn(data, _, _)
            | Self::Knight(data)
            | Self::Bishop(data)
            | Self::Rook(data, _)
            | Self::Queen(data)
            | Self::King(data, _) => data.position,
        }
    }

    /// Returns a PieceType value according to the piece's type.
    pub fn get_type(&self) -> PieceType {
        match *self {
            Self::Pawn(_, _, _) => PieceType::Pawn,
            Self::Knight(_) => PieceType::Knight,
            Self::Bishop(_) => PieceType::Bishop,
            Self::Rook(_, _) => PieceType::Rook,
            Self::Queen(_) => PieceType::Queen,
            Self::King(_, _) => PieceType::King,
        }
    }

    /// Returns true if a pawn is in a circumstance where it can be taken
    /// en-passant by another pawn.
    pub fn is_en_passantable(&self) -> bool {
        if let Self::Pawn(_, _, result) = *self {
            result
        } else {
            unreachable!("This method should be used on a pawn.")
        }
    }
    /// Returns true if a king or a rook hasn't yet moved an is therefore
    /// eligible for castling.
    pub fn can_castle(&self) -> bool {
        match *self {
            Self::King(_, result) | Self::Rook(_, result) => result,
            _ => unreachable!("This method should be used on a king or a rook."),
        }
    }
    /// Checks if the pawn is about to make it's first move; if so - returns
    /// true.
    pub fn first_move(&self) -> bool {
        if let Self::Pawn(_, result, _) = *self {
            result
        } else {
            unreachable!("This method should be used on a pawn.")
        }
    }
    fn get_moves_lines(&self, directions: Vec<(i32, i32)>, state: &GameState) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];
        for (dir_column, dir_row) in directions {
            for distance in 1..8 {
                let new_column =
                    ((self.get_position().get_column() as i32) + distance * dir_column) as u8;
                let new_row = ((self.get_position().get_row() as i32) + distance * dir_row) as u8;
                if let Some(position) = Position::new(new_row, new_column) {
                    if let Some(other_piece) = state.get_piece(position) {
                        if other_piece.get_player() != self.get_player() {
                            result.push(Move::new(self.get_position(), position));
                        }
                        break;
                    } else {
                        result.push(Move::new(self.get_position(), position));
                    }
                }
            }
        }
        result
    }
    fn get_moves_shifts(&self, shifts: Vec<(i32, i32)>, state: &GameState) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];
        for (column_shift, row_shift) in shifts {
            let new_column = (self.get_position().get_column() as i32 + column_shift) as u8;
            let new_row = (self.get_position().get_row() as i32 + row_shift) as u8;
            if let Some(new_pos) = Position::new(new_column, new_row) {
                if let Some(other_piece) = state.get_piece(new_pos) {
                    if other_piece.get_player() != self.get_player() {
                        result.push(Move::new(self.get_position(), new_pos));
                    }
                }
            }
        }
        result
    }
    /// Returns a vector of moves possible to make for a given piece given a
    /// game state object. The resulting moves are guaranteed to be legal
    /// according to the rules of the game.
    pub fn get_moves(&self, state: &GameState) -> Vec<Move> {
        let result = vec![];
        match *self {
            Self::Pawn(_, first_move, is_en_passantable) => {
                unimplemented!()
            }
            Self::Knight(PieceData {
                position: _,
                player: _,
            }) => {
                let mut possible_shifts = iproduct!([-2, 2], [-1, 1]).collect_vec();
                possible_shifts.extend(iproduct!([-1, 1], [-2, 2]));
                self.get_moves_shifts(possible_shifts, state)
            }
            Self::Bishop(_) => {
                let directions = iproduct!([1, -1], [1, -1]).collect_vec();
                self.get_moves_lines(directions, state)
            }
            Self::Rook(_, _) => {
                let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
                self.get_moves_lines(directions, state)
            }
            Self::Queen(_) => {
                let directions = iproduct!([1, 0, -1], [1, 0, -1]).collect_vec();
                self.get_moves_lines(directions, state)
            }
            Self::King(_, can_castle) => {
                let shifts = iproduct!([1, 0, -1], [1, 0, -1])
                    .filter(|&x| x != (0, 0))
                    .collect_vec();
                let mut result = self.get_moves_shifts(shifts, state);
                let (row, _) = self.get_position().to_tuple();
                // TODO: add rules for when enemy pieces attack the squares
                // between the king and the rook
                if can_castle {
                    if let Some(Self::Rook(
                        PieceData {
                            position: _,
                            player,
                        },
                        true,
                    )) = state.get_piece(Position::new(row, 0).unwrap())
                    {
                        if *player == self.get_player()
                            && (1..=3)
                                .map(|idx| state.get_piece(Position::new(row, idx).unwrap()))
                                .all(|piece| piece.is_none())
                        {
                            result.push(Move::new(
                                self.get_position(),
                                Position::new(row, 2).unwrap(),
                            ));
                        }
                    }
                    if let Some(Self::Rook(
                        PieceData {
                            position: _,
                            player,
                        },
                        true,
                    )) = state.get_piece(Position::new(row, 7).unwrap())
                    {
                        if *player == self.get_player()
                            && (5..=6)
                                .map(|idx| state.get_piece(Position::new(row, idx).unwrap()))
                                .all(|piece| piece.is_none())
                        {
                            result.push(Move::new(
                                self.get_position(),
                                Position::new(row, 6).unwrap(),
                            ));
                        }
                    }
                }
                result
            }
        };
        result
    }
}