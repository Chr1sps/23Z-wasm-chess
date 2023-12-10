pub use crate::game::Game;
pub use crate::game::chessfield::chessman;
pub use crate::game::chessfield::chessman::Chessman;
pub use crate::game::chessfield::chessman::ChessmanStatus;
pub use crate::game::chessfield::chessman::basic::PlayerKind;
pub use crate::game::chessfield::chessman::basic::Position;
pub mod game;


fn main() {
    let mut game = Game::new();
    println!("Current player: {:?}", game.current_player);
    game.set_field((4, 4), chessman::Rook::new(Position{row: 4, column: 4}, PlayerKind::White, ChessmanStatus::NotCaptured));
    game.make_move((4, 4));
}
