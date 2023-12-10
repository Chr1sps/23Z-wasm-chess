pub use basic::PlayerKind;
pub use basic::Position;
use itertools::iproduct;


pub mod basic;

#[derive(Debug)]
pub struct ChessMove {
    pub current_position: Position,
    pub end_position: Position,
}

impl ChessMove{
    pub fn get_end_position(&self) -> (i32, i32){
        (self.end_position.row, self.end_position.column)
    }

    pub fn get_current_position(&self) -> (i32, i32){
        (self.current_position.row, self.current_position.column)
    }
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

pub trait ChessPieceTrait{

    fn new_chessman(new_position: Position, new_player: PlayerKind, new_status: ChessmanStatus) -> Chessman where Self: Sized{
        Chessman{
            position: new_position,
            player: new_player, 
            status: new_status,
        }
    }

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

    fn calculate_params(&self) -> Vec<(i32, i32)> {
        let a = vec![1, 0, -1];
        let b = vec![1, 0, -1];
        let mut params: Vec<(i32, i32)> = iproduct!(a, b).collect();
        params.retain(|&x| x != (0, 0));
        params
    }

    fn get_moves(&self) -> Vec<ChessMove>;

    fn get_player(&self) -> &PlayerKind;

    fn get_position(&self) -> &Position;
}

// struct Pawn {
//     chessman: Chessman,
// }


// impl ChessPieceTrait
// Trait for Pawn {
//     pub fn get_moves(&self) -> &str {"Move of Pawn"}

// }

pub struct Rook {
    pub chessman: Chessman,
}

impl Rook {
    pub fn new(position: Position, player: PlayerKind, status: ChessmanStatus) -> Rook {
        Rook{
            chessman: <Rook as ChessPieceTrait>::new_chessman(position, player, status),
        }
    }
}

impl ChessPieceTrait for Rook {
    fn get_moves(&self) -> Vec<ChessMove> {
        let directions:  Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        self.get_moves_lines(directions)
    }

    fn get_player(&self) -> &PlayerKind{
        &self.chessman.player
    }

    fn get_position(&self) -> &Position{
        &self.chessman.position
    }

}

pub struct King {
    pub chessman: Chessman,
}

impl ChessPieceTrait for King {
    fn get_moves(&self) -> Vec<ChessMove> {
        let possible_shifts: Vec<(i32, i32)> = self.calculate_params();
        self.get_moves_shifts(possible_shifts)
    }

    fn get_player(&self) -> &PlayerKind{
        &self.chessman.player
    }

    fn get_position(&self) -> &Position{
        &self.chessman.position
    }
}

pub struct Queen {
    pub chessman: Chessman,
}

impl ChessPieceTrait for Queen {
    fn get_moves(&self) -> Vec<ChessMove> {
        let directions: Vec<(i32, i32)> = self.calculate_params();
        return self.get_moves_lines(directions);

    }

    fn get_player(&self) -> &PlayerKind{
        &self.chessman.player
    }

    fn get_position(&self) -> &Position{
        &self.chessman.position
    }

}

pub struct Bishop {
    pub chessman: Chessman,
}

impl ChessPieceTrait for Bishop {
    fn get_moves(&self) -> Vec<ChessMove> {
        let a = vec![-1, 1];
        let b = vec![-1, 1];
        return self.get_moves_lines(iproduct!(a, b).collect());
    }

    fn get_player(&self) -> &PlayerKind{
        &self.chessman.player
    }

    fn get_position(&self) -> &Position{
        &self.chessman.position
    }


}

pub struct Knight {
    pub chessman: Chessman,
}

impl ChessPieceTrait for Knight {
    fn get_moves(&self) -> Vec<ChessMove> {
        let possible_shifts: Vec<(i32, i32)> = self.calculate_params();
        return self.get_moves_shifts(possible_shifts);

    }

    fn calculate_params(&self) -> Vec<(i32, i32)>{
        let a = vec![-2, 2];
        let b = vec![-1, 1];
        let mut possible_shifts: Vec<(i32, i32)> = iproduct!(a.clone(), b.clone()).collect();
        possible_shifts.extend(iproduct!(b, a).collect::<Vec<(i32, i32)>>());
        possible_shifts
    }

    fn get_player(&self) -> &PlayerKind{
        &self.chessman.player
    }

    fn get_position(&self) -> &Position{
        &self.chessman.position
    }

}
