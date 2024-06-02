use strum::EnumIter;

/// Represents the suits of a card.
#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardSuits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
