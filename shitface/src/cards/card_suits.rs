use strum::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum CardSuits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
