use super::card_suits::CardSuits;

/// Represents the color of a card.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardColor {
    Red,
    Black,
}

impl From<CardSuits> for CardColor {
    /// Converts a CardSuits enum variant into a corresponding CardColor enum variant.
    ///
    /// # Arguments
    ///
    /// * `value` - The CardSuits enum variant to convert.
    ///
    /// # Returns
    ///
    /// The corresponding CardColor enum variant.
    fn from(value: CardSuits) -> Self {
        use super::card_suits::CardSuits::{Clubs, Diamonds, Hearts, Spades};

        match value {
            Spades | Clubs => Self::Black,
            Hearts | Diamonds => Self::Red,
        }
    }
}
