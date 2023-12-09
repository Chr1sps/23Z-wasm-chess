pub use basic::PlayerKind;
pub use basic::Position;

pub mod basic;

pub enum ChessmanStatus {
    Captured,
    NotCaptured,
}

pub struct Chessman {
    position: Position,
    player: PlayerKind, 
    status: ChessmanStatus,
}

pub trait Chesspiece {
    fn get_moves(&self) -> &str;
    //     result = Vec<>;
    //     let new column

    // };

    // fn get_moves_shifts(&self) -> &str{

    // };
}

struct Pawn {
    chessman: Chessman,
}


impl Chesspiece for Pawn {
    fn get_moves(&self) -> &str {"Move of Pawn"}

}

struct Rook {
    chessman: Chessman,
}

impl Chesspiece for Rook {
    fn get_moves(&self) -> &str {"Move of Rook"}

}

struct King {
    chessman: Chessman,
}

impl Chesspiece for King {
    fn get_moves(&self) -> &str {"Move of King"}

}

struct Queen {
    chessman: Chessman,
}

impl Chesspiece for Queen {
    fn get_moves(&self) -> &str {"Move of Queen"}

}

struct Bishop {
    chessman: Chessman,
}

impl Chesspiece for Bishop {
    fn get_moves(&self) -> &str {"Move of Bishop"}

}

struct Knight {
    chessman: Chessman,
}

impl Chesspiece for Knight {
    fn get_moves(&self) -> &str {"Move of Knight"}

}
