use strum::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardSuits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
