use crate::gamestate::GameState;
pub use crate::player::Player;
pub use crate::position::Position;
pub use crate::r#move::Move;
use itertools::{iproduct, Itertools};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// This struct contains data that is present in every Piece enum variant,
/// namely the position of a piece and which player does it belong to.
#[derive(Clone, Copy, Debug)]
pub struct SharedData {
    position: Position,
    player: Player,
}

impl SharedData {
    pub fn new(new_position: Position, new_player: Player) -> SharedData {
        SharedData {
            position: new_position,
            player: new_player,
        }
    }
    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_player(&self) -> Player {
        self.player
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    /// The pawn variant; contains a field which indicates if a pawn is yet to
    /// make it's first move, and if it's therefore able to make a move forward
    /// by two squares.
    Pawn(SharedData, bool),
    Knight(SharedData),
    Bishop(SharedData),
    /// The rook variant; contains a field which indicates if the rook can be
    /// used for castling.
    Rook(SharedData, bool),
    Queen(SharedData),
    /// The king variant; contains a field which indicates if the rook can be
    /// used for castling.
    King(SharedData, bool),
}

impl Piece {
    /// Creates a new pawn.
    pub fn new_pawn(position: Position, player: Player, first_move: bool) -> Self {
        Self::Pawn(SharedData::new(position, player), first_move)
    }
    /// Creates a new knight.
    pub fn new_knight(position: Position, player: Player) -> Self {
        Self::Knight(SharedData::new(position, player))
    }
    /// Creates a new bishop.
    pub fn new_bishop(position: Position, player: Player) -> Self {
        Self::Bishop(SharedData::new(position, player))
    }
    /// Creates a new rook.
    pub fn new_rook(position: Position, player: Player, can_castle: bool) -> Self {
        Self::Rook(SharedData::new(position, player), can_castle)
    }
    /// Creates a new queen.
    pub fn new_queen(position: Position, player: Player) -> Self {
        Self::Queen(SharedData::new(position, player))
    }
    /// Creates a new king.
    pub fn new_king(position: Position, player: Player, can_castle: bool) -> Self {
        Self::King(SharedData::new(position, player), can_castle)
    }

    /// Returns the piece's player.
    pub fn get_player(&self) -> Player {
        match self {
            Self::Pawn(data, _)
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
            Self::Pawn(data, _)
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
            Self::Pawn(_, _) => PieceType::Pawn,
            Self::Knight(_) => PieceType::Knight,
            Self::Bishop(_) => PieceType::Bishop,
            Self::Rook(_, _) => PieceType::Rook,
            Self::Queen(_) => PieceType::Queen,
            Self::King(_, _) => PieceType::King,
        }
    }
    pub fn get_data(&self) -> PieceData {
        let player = match *self {
            Self::Pawn(
                SharedData {
                    position: _,
                    player,
                },
                _,
            )
            | Self::Knight(SharedData {
                position: _,
                player,
            })
            | Self::Bishop(SharedData {
                position: _,
                player,
            })
            | Self::Rook(
                SharedData {
                    position: _,
                    player,
                },
                _,
            )
            | Self::Queen(SharedData {
                position: _,
                player,
            })
            | Self::King(
                SharedData {
                    position: _,
                    player,
                },
                _,
            ) => player,
        };
        PieceData::new(player, self.get_type())
    }

    /// Returns true if a pawn is in a circumstance where it can be taken
    /// en-passant by another pawn.
    // pub fn is_en_passantable(&self) -> bool {
    //     if let Self::Pawn(_, result) = *self {
    //         result
    //     } else {
    //         unreachable!("This method should be used on a pawn.")
    //     }
    // }
    /// Returns true if a king or a rook hasn't yet moved an is therefore
    /// eligible for castling.
    pub fn can_castle(&self) -> bool {
        match *self {
            Self::King(_, result) | Self::Rook(_, result) => result,
            _ => false,
        }
    }
    /// Checks if the pawn is about to make it's first move; if so - returns
    /// true.
    pub fn first_move(&self) -> bool {
        if let Self::Pawn(_, result) = *self {
            result
        } else {
            false
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
        for (row_shift, column_shift) in shifts {
            let new_column = (self.get_position().get_column() as i32 + column_shift) as u8;
            let new_row = (self.get_position().get_row() as i32 + row_shift) as u8;
            if let Some(new_pos) = Position::new(new_row, new_column) {
                if let Some(other_piece) = state.get_piece(new_pos) {
                    if other_piece.get_player() != self.get_player() {
                        result.push(Move::new(self.get_position(), new_pos));
                    }
                } else {
                    result.push(Move::new(self.get_position(), new_pos));
                }
            }
        }
        result
    }
    fn check_castle_move(&self, state: &GameState, right_side: bool) -> Option<Move> {
        let (row, _) = self.get_position().as_tuple();
        if let Some(Self::Rook(
            SharedData {
                position: _,
                player,
            },
            true,
        )) = state.get_piece(Position::new(row, if right_side { 7 } else { 0 }).unwrap())
        {
            if *player == self.get_player()
                && if right_side { 5..=6 } else { 1..=3 }
                    .map(|idx| state.get_piece(Position::new(row, idx).unwrap()))
                    .all(|piece| piece.is_none())
            {
                return Some(Move::new(
                    self.get_position(),
                    Position::new(row, if right_side { 6 } else { 2 }).unwrap(),
                ));
            }
        }
        None
    }
    /// Returns a vector of moves possible to make for a given piece given a
    /// game state object. The resulting moves are guaranteed to be legal
    /// according to the rules of the game.
    pub fn get_moves(&self, state: &GameState) -> Vec<Move> {
        let result = self.get_unchecked_moves(state);
        result
            .into_iter()
            .filter(move |m| {
                let new_state = GameState::transform_state(state, *m, None);
                let new_state = new_state.switch_player();
                !new_state.is_checked()
            })
            .collect_vec()
    }
    /// Returns all the moves that a piece can *physically* make, therefore not
    /// taking into account possible consequences such as checks, pins, etc.
    pub fn get_unchecked_moves(&self, state: &GameState) -> Vec<Move> {
        match *self {
            Self::Pawn(_, first_move) => {
                let (row, col) = self.get_position().as_tuple();

                let idx: i8;
                if self.get_player() == Player::White {
                    idx = 1;
                } else {
                    idx = -1;
                }
                println!("first move: {}", first_move);
                let mut result = vec![];
                if let Some(to_pos) = Position::new((row as i8 + idx).try_into().unwrap(), col) {
                    println!("to_pos: {:?}", to_pos);
                    if let None = state.get_piece(to_pos) {
                        println!("pushing");
                        result.push(Move::new(self.get_position(), to_pos));
                    }
                }
                if first_move {
                    if let Some(to_pos) =
                        Position::new((row as i8 + 2 * idx).try_into().unwrap(), col)
                    {
                        if let None = state.get_piece(to_pos) {
                            result.push(Move::new(self.get_position(), to_pos));
                        }
                    }
                }
                if col >= 1 {
                    if let Some(to_pos) =
                        Position::new((row as i8 + idx).try_into().unwrap(), col - 1)
                    {
                        if let Some(piece) = state.get_piece(to_pos) {
                            if piece.get_player() != self.get_player() {
                                result.push(Move::new(self.get_position(), to_pos));
                            }
                        }
                    }
                    if let Some(piece) = state.get_piece(Position::new(row, col - 1).unwrap()) {
                        if state.get_en_passant_square()
                            == Some(&Position::new(row, col - 1).unwrap())
                        {
                            println!("en passant");
                            if piece.get_player() != self.get_player() {
                                println!("pushing");
                                if let Some(to_pos) =
                                    Position::new((row as i8 + idx).try_into().unwrap(), col - 1)
                                {
                                    result.push(Move::new(self.get_position(), to_pos));
                                }
                            }
                        }
                    }
                }
                if col <= 6 {
                    if let Some(to_pos) = Position::new((row as i8 + idx).try_into().unwrap(), col + 1)
                    {
                        if let Some(piece) = state.get_piece(to_pos) {
                            if piece.get_player() != self.get_player() {
                                result.push(Move::new(self.get_position(), to_pos));
                            }
                        }
                    }

                    if let Some(piece) = state.get_piece(Position::new(row, col + 1).unwrap()) {
                        if state.get_en_passant_square() == Some(&Position::new(row, col + 1).unwrap())
                        {
                            if piece.get_player() != self.get_player() {
                                if let Some(to_pos) =
                                    Position::new((row as i8 + idx).try_into().unwrap(), col + 1)
                                {
                                    result.push(Move::new(self.get_position(), to_pos));
                                }
                            }
                        }
                    }
            }

                result
            }
            Self::Knight(SharedData {
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
                let directions = iproduct!([1, 0, -1], [1, 0, -1])
                    .filter(|&x| x != (0, 0))
                    .collect_vec();
                self.get_moves_lines(directions, state)
            }
            Self::King(_, can_castle) => {
                let shifts = iproduct!([1, 0, -1], [1, 0, -1])
                    .filter(|&x| x != (0, 0))
                    .collect_vec();
                let mut result = self.get_moves_shifts(shifts, state);

                if can_castle {
                    if let Some(chess_move) = self.check_castle_move(state, false) {
                        result.push(chess_move);
                    }
                    if let Some(chess_move) = self.check_castle_move(state, true) {
                        result.push(chess_move);
                    }
                }
                result
            }
        }
    }
    pub fn shift(self, new_pos: Position) -> Self {
        match self {
            Self::Pawn(
                SharedData {
                    position: _,
                    player,
                },
                _,
            ) => Self::new_pawn(new_pos, player, false),
            Self::Knight(SharedData {
                position: _,
                player,
            }) => Self::new_knight(new_pos, player),
            Self::Bishop(SharedData {
                position: _,
                player,
            }) => Self::new_bishop(new_pos, player),
            Self::Rook(
                SharedData {
                    position: _,
                    player,
                },
                _,
            ) => Self::new_pawn(new_pos, player, false),
            Self::Queen(SharedData {
                position: _,
                player,
            }) => Self::new_queen(new_pos, player),
            Self::King(
                SharedData {
                    position: _,
                    player,
                },
                _,
            ) => Self::new_king(new_pos, player, false),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct PieceData {
    player: Player,
    piece_type: PieceType,
}

#[wasm_bindgen]
impl PieceData {
    pub fn new(player: Player, piece_type: PieceType) -> Self {
        Self { player, piece_type }
    }
    pub fn get_player(&self) -> Player {
        self.player
    }
    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }
}
