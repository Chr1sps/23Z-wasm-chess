pub use crate::game::Game;
pub use crate::game::chessfield::chessman;
pub use crate::game::chessfield::chessman::Chessman;
pub use crate::game::chessfield::chessman::ChessmanKind;
pub use crate::game::chessfield::chessman::ChessmanStatus;
pub use crate::game::chessfield::chessman::basic::PlayerKind;
pub use crate::game::chessfield::chessman::basic::Position;
pub mod game;


fn main() {
    let mut game = Game::new();
    game.set_field((4, 4), chessman::Pawn::new(Position{row: 4, column: 4}, PlayerKind::White, ChessmanStatus::NotCaptured, ChessmanKind::Pawn));
    let _ =game.make_pawn_move((4, 4));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_rook_movement() {
        let mut game = Game::new();
        game.set_field((4, 4), chessman::Rook::new(Position{row: 4, column: 4}, PlayerKind::White, ChessmanStatus::NotCaptured, ChessmanKind::Rook));
        let result  = game.make_move((4, 4));
        assert_eq!(result.unwrap().len(), 14);
    }

    #[test]
    fn test_queen_movement() {
        let mut game = Game::new();
        game.set_field((4, 4), chessman::Queen::new(Position{row: 4, column: 4}, PlayerKind::White, ChessmanStatus::NotCaptured, ChessmanKind::Queen));
        let result  = game.make_move((4, 4));
        assert_eq!(result.unwrap().len(), 27);
    }

    #[test]
    fn test_king_movement() {
        let mut game = Game::new();
        game.set_field((4, 4), chessman::King::new(Position{row: 4, column: 4}, PlayerKind::White, ChessmanStatus::NotCaptured, ChessmanKind::King));
        let result  = game.make_move((4, 4));
        assert_eq!(result.unwrap().len(), 8);
    }

    #[test]
    fn test_bishop_movement() {
        let mut game = Game::new();
        game.set_field((4, 4), chessman::Bishop::new(Position{row: 4, column: 4}, PlayerKind::White, ChessmanStatus::NotCaptured, ChessmanKind::Bishop));
        let result  = game.make_move((4, 4));
        assert_eq!(result.unwrap().len(), 13);
    }

    #[test]
    fn test_knight_movement() {
        let mut game = Game::new();
        game.set_field((4, 4), chessman::Knight::new(Position{row: 4, column: 4}, PlayerKind::White, ChessmanStatus::NotCaptured, ChessmanKind::Knight));
        let result  = game.make_move((4, 4));
        assert_eq!(result.unwrap().len(), 8);
    }

    #[test]
    fn test_pawn_movement() {
        let mut game = Game::new();
        game.set_field((4, 4), chessman::Pawn::new(Position{row: 4, column: 4}, PlayerKind::White, ChessmanStatus::NotCaptured, ChessmanKind::Pawn));
        let result  = game.make_pawn_move((4, 4));
        assert_eq!(result.unwrap().len(), 2);
    }



}

