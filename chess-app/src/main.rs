pub use crate::game::Game;
pub mod game;

fn main() {
    let game = Game::new();
    println!("Current player: {:?}", game.current_player);
}
