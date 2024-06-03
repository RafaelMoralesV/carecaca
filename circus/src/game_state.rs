use shitface::{deck::Deck, player::Player};

/// Indicates the state of the Game.
pub struct GameState {
    /// The list of players currently participating in the game.
    pub players: Vec<Player>,

    /// The deck of cards used in the game.
    pub deck: Deck,

    /// The current pile of played cards.
    pub pile: Deck,

    /// The direction the game is currently following.
    pub direction: Direction,

    /// Index of the currently playing player.
    current_player: usize,
}

/// Represents the game direction.
pub enum Direction {
    Clockwise,
    CounterClockwise,
}
