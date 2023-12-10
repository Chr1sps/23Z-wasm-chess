pub use chessfield::chessman::basic::PlayerKind;
pub use chessfield::chessman::basic::Position;
pub use chessfield::chessman::ChessPieceTrait;
pub use chessfield::chessman::ChessMove;
pub use chessfield::ChessField;
use std::ops::Not;

pub mod chessfield;

pub enum Status {
    Check,
    Checkmate,
    Normal,
}

pub struct Game {
    pub current_player: PlayerKind,
    pub other_player: PlayerKind,
    pub status: Status,
    pub chessboard: Vec<Vec<ChessField>>,

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

    pub fn make_move(&self, position: (i32, i32)){
        let (row, column) = position;
        let mut filtered_results: Vec<ChessMove> = vec![];
        let chessfield: &ChessField = self.get_field(row, column);
        match chessfield.get_status() {
            None => println!("You must select field with chesspiece"),
            Some(chessman) => {
                let results: Vec<ChessMove>= chessman.get_moves();
                for result in results{
                    println!("{}", result.end_position.row);
                    let (new_row, new_column)= result.get_end_position();
                    let possible_field: &ChessField = self.get_field(new_row, new_column);
                    if (possible_field.get_status().is_some()
                            && chessman.get_player() ==  possible_field.get_status().as_ref().unwrap().get_player()).not(){
                                filtered_results.push(result);
                    }
                }
                println!("{:#?}", filtered_results)
            },
        }
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