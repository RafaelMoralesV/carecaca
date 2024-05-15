use super::card_suits::CardSuits;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardColor {
    Red,
    Black,
}

impl From<CardSuits> for CardColor {
    fn from(value: CardSuits) -> Self {
        use super::card_suits::CardSuits::*;

        match value {
            Spades | Clubs => CardColor::Black,
            Hearts | Diamonds => CardColor::Red,
        }
    }
}
