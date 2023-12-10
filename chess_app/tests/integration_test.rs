// pub use super::*;
// pub use crate::game::chessfield::chessman;
// pub use crate::game::chessfield::chessman::Chessman;
// pub use crate::game::chessfield::chessman::ChessmanStatus;
// pub use crate::game::chessfield::chessman::basic::PlayerKind;
// pub use crate::game::chessfield::chessman::basic::Position;

// #[test]
// fn test_Rook_moves() {
//     let mut game = Game::new();
//     game.set_field((4, 4), chessman::Rook::new(Position{row: 4, column: 4}, PlayerKind::White, ChessmanStatus::NotCaptured));
//     let result: Vec<Game::ChessMove> = game.make_move((4, 4));
//     assert_eq!(result.len(), 14);
// }