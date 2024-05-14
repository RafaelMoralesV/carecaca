use strum::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
    Ace,
    Numeric(u8),
    Jack,
    Queen,
    King,
    Joker,
}
