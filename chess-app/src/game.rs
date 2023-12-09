pub use chessfield::chessman::basic::PlayerKind;
pub use chessfield::chessman::basic::Position;
pub use chessfield::ChessField;

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
}