use crate::cards::card::Card;

/// Represents a player in the game.
pub struct Player {
    /// The hand of the player, which is hidden information from other players.
    pub hand: Vec<Card>,

    /// Face-up cards of the players, which is public information for everyone.
    pub face_up: Vec<Card>,

    /// Face-down cards of the player, which is hidden information for everyone.
    pub face_down: Vec<Card>,

    /// Indicates whether the player is out of the game.
    pub is_out: bool,
}
