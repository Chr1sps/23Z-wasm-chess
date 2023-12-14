pub use chessfield::chessman::basic::PlayerKind;
pub use chessfield::chessman::basic::Position;
pub use chessfield::chessman::ChessPieceTrait;
pub use chessfield::chessman::ChessMove;
pub use chessfield::chessman::ChessmanKind;
pub use chessfield::ChessField;
use std::ops::Not;
use std::mem;

pub mod chessfield;

#[derive(Debug)]
pub struct MyError {
    pub message: String,
}

pub enum Status {
    Check,
    Checkmate,
    Normal,
}

pub struct Game {
    current_player: PlayerKind,
    other_player: PlayerKind,
    pub status: Status,
    chessboard: Vec<Vec<ChessField>>,

}

impl Game {
    pub fn new() -> Self {
        Self{
            current_player: PlayerKind::White,
            other_player: PlayerKind::Black,
            status: Status::Normal,
            chessboard: Game::init_board(),
        
        }
    }

    pub fn get_game_status(&self) -> &Status {
        &self.status
    }

    pub fn set_game_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    pub fn get_current_player(&self) -> &PlayerKind{
        &self.current_player
    }

    pub fn get_other_player(&self) -> &PlayerKind{
        &self.other_player
    }

    pub fn change_round(&mut self) {
        mem::swap(&mut self.current_player, &mut self.other_player)
    }

    fn init_board() -> Vec<Vec<ChessField>>{
        let mut chessboard: Vec<Vec<ChessField>> = Vec::with_capacity(8);

        for i in 0..8 {
            let mut row: Vec<ChessField> = Vec::with_capacity(8);
            for j in 0..8 {
                row.push(ChessField::new(Position {row: i, column: j}));
            }
            chessboard.push(row);
        }
        chessboard
    }

    pub fn make_move(&self, position: (i32, i32)) -> Result<Vec<ChessMove>, MyError> {
        let (row, column) = position;
        let mut filtered_results: Vec<ChessMove> = vec![];
        let chessfield: &ChessField = self.get_field(row, column);
        match chessfield.get_status() {
            None => Err(MyError{message: "You must select field with chesspiece".to_string()}),
            Some(chessman) => {
                let results: Vec<ChessMove>= chessman.get_moves();
                for result in results{
                    let (new_row, new_column)= result.get_end_position();
                    let possible_field: &ChessField = self.get_field(new_row, new_column);
                    if (possible_field.get_status().is_some()
                            && chessman.get_player() ==  possible_field.get_status().as_ref().unwrap().get_player()).not(){
                                filtered_results.push(result);
                    }
                }
                println!("{:#?}", filtered_results);
                println!("{}", filtered_results.len());
                Ok(filtered_results)
            },
        }
    }

    pub fn make_pawn_move(&self, start_position: (i32, i32)) -> Result<Vec<ChessMove>, MyError> {

        let mut results: Vec<ChessMove> = vec![];

        let (row, column) = start_position;
        let right_column = column + 1;
        let left_column = row - 1;
        let row_shift = if self.get_current_player() == &PlayerKind::White { 1 } else { -1 };

        let chessfield: &ChessField = self.get_field(row, column);
        
        if self.get_field(row + row_shift, column).get_status().is_none(){
            let chessmove = chessfield.get_status().as_ref().unwrap().create_move(column, row + row_shift);
            results.push(chessmove.unwrap());
        }

        if *chessfield.get_status().as_ref().unwrap().is_first_move() 
           && self.get_field(row + row_shift, column).get_status().is_none()
           && self.get_field(row + 2 *row_shift, column).get_status().is_none(){
            let chessmove = chessfield.get_status().as_ref().unwrap().create_move(column, row + 2 * row_shift);
            results.push(chessmove.unwrap());
        }

        if right_column >= 0 && right_column < 8 {
            let right_diagonal = self.get_field(row + row_shift, right_column).get_status();
            if right_diagonal.is_some()
               && right_diagonal.as_ref().unwrap().get_player() != self.get_current_player(){
                let chessmove = chessfield.get_status().as_ref().unwrap().create_move(right_column, row + row_shift);
                results.push(chessmove.unwrap());
            }
            // is_en_passantable

        }

        if left_column >= 0 && left_column < 8 {
            let left_diagonal = self.get_field(row + row_shift, left_column).get_status();
            if left_diagonal.is_some()
               && left_diagonal.as_ref().unwrap().get_player() != self.get_current_player(){
                let chessmove = chessfield.get_status().as_ref().unwrap().create_move(left_column, row + row_shift);
                results.push(chessmove.unwrap());
               }
            // is_en_passantable
        }
        println!("{:#?}", results);
        println!("{}", results.len());
        Ok(results)
    }

    pub fn set_field<T: ChessPieceTrait + 'static>(&mut self, position: (i32, i32), chesspiece: T ) {
        let (row, column) = position;
        let row_i = row as usize;
        let column_i = column as usize;
        if let Some(field) = self.chessboard.get_mut(row_i).and_then(|row| row.get_mut(column_i)) {
            field.set_status(Some(Box::new(chesspiece)));
        }
    }

    pub fn get_field(&self, row: i32, column: i32) -> &ChessField{
        let idx_row = row as usize;
        let idx_column = column as usize;
        &self.chessboard[idx_row][idx_column]
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_change_round(){
        let mut game = Game::new();
        game.change_round();
        assert_eq!(game.get_current_player(), &PlayerKind::Black);
        assert_eq!(game.get_other_player(), &PlayerKind::White);
    }
}