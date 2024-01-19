pub struct Game{
    gameState: GameState,
    player1: Player,
    player2: Player,

}
impl Game {
    pub fn init() -> Self {
        Game{
            gameState: GameState::first_state(),
            player1: Player::White,
            player2: Player::Black,
        }
    }
}