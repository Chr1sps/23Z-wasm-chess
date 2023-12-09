pub use basic::PlayerKind;
pub use basic::Position;

pub mod basic;

#[derive(Debug)]
pub struct ChessMove {
    pub current_position: Position,
    pub end_position: Position,
}

pub enum ChessmanStatus {
    Captured,
    NotCaptured,
}

pub struct Chessman {
    pub position: Position,
    pub player: PlayerKind, 
    pub status: ChessmanStatus,
}

pub trait Chesspiece {
    fn get_moves_lines(&self, directions: Vec<(i32, i32)>) -> Vec<ChessMove>{
        let mut results: Vec<ChessMove> = vec![];
        for direction in directions{
            let (dir_row, dir_column) = direction;
            for distance in 1..8{
                let new_column = self.get_position().column + distance * dir_column;
                let new_row = self.get_position().row + distance * dir_row;
                match self.create_move(new_column, new_row) {
                    None => continue,
                    Some(chessmove) => results.push(chessmove),
                }
            }
        }
        results

    }

    fn get_moves_shifts(&self, positions: Vec<(i32, i32)>) ->  Vec<ChessMove>{
        let mut result: Vec<ChessMove> = vec![];
        for position in positions{
            let new_column = self.get_position().column + position.0;
            let new_row = self.get_position().row + position.1;
            match self.create_move(new_column, new_row) {
                None => continue,
                Some(chessmove) => result.push(chessmove),
            }
        }
        result
    }



    fn create_move(&self, new_column: i32, new_row: i32) -> Option<ChessMove>{
        if new_column < 0 || new_column >= 8 || new_row < 0 || new_row >= 8{
            return None;
        }
        Some(ChessMove{current_position:
                    Position {
                                row: self.get_position().row,
                                column: self.get_position().column},
                    end_position:
                    Position {
                                row: new_row,

                                    column: new_column}
                })

    }

    fn get_moves(&self) -> Vec<ChessMove>;

    fn get_player(&self) -> &PlayerKind;

    fn get_position(&self) -> &Position;
}

// struct Pawn {
//     chessman: Chessman,
// }


// impl Chesspiece for Pawn {
//     pub fn get_moves(&self) -> &str {"Move of Pawn"}

// }

pub struct Rook {
    pub chessman: Chessman,
}

impl Chesspiece for Rook {
    fn get_moves(&self) -> Vec<ChessMove> {
        let directions:  Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        return self.get_moves_lines(directions);
    }

    fn get_player(&self) -> &PlayerKind{
        &self.chessman.player
    }

    fn get_position(&self) -> &Position{
        &self.chessman.position
    }

}

// struct King {
//     chessman: Chessman,
// }

// impl Chesspiece for King {
//     fn get_moves(&self) -> &str {"Move of King"}

// }

// struct Queen {
//     chessman: Chessman,
// }

// impl Chesspiece for Queen {
//     fn get_moves(&self) -> &str {"Move of Queen"}

// }

// struct Bishop {
//     chessman: Chessman,
// }

// impl Chesspiece for Bishop {
//     fn get_moves(&self) -> &str {"Move of Bishop"}

// }

// struct Knight {
//     chessman: Chessman,
// }

// impl Chesspiece for Knight {
//     fn get_moves(&self) -> &str {"Move of Knight"}

// }
