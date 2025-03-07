use super::{card_color::CardColor, card_rank::CardRank, card_suits::CardSuits};

/// Represents a playing card.
///
/// A card must have a Rank and a Suit. The color of the card is derived from it.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: CardRank,
    pub color: CardColor,
    pub suit: CardSuits,
}

impl Card {
    /// Constructs a new card with the given rank and suit.
    ///
    /// # Arguments
    ///
    /// * `rank` - The rank of the card.
    /// * `suit` - The suit of the card.
    ///
    /// # Returns
    ///
    /// A Result containing the constructed Card if successful, otherwise an error.
    ///
    /// # Errors
    ///
    /// Returns an error if the card rank is invalid.
    pub fn new(rank: CardRank, suit: CardSuits) -> Self {
        Self {
            rank,
            color: suit.into(),
            suit,
        }
    }
}
