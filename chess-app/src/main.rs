pub use crate::game::Game;
pub use crate::game::chessfield::chessman::Rook;
pub use crate::game::chessfield::chessman::Chessman;
pub use crate::game::chessfield::chessman::ChessmanStatus;
pub use crate::game::chessfield::chessman::basic::PlayerKind;
pub use crate::game::chessfield::chessman::basic::Position;
pub mod game;


fn main() {
    let mut game = Game::new();
    println!("Current player: {:?}", game.current_player);
    game.set_field((0, 0), Rook{chessman: Chessman{
        position: Position{row: 0, column: 0},
        player: PlayerKind::White,
        status: ChessmanStatus::NotCaptured,
    }});
    game.make_move((0, 0));
}
