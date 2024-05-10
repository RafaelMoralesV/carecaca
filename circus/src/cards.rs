use std::{error, fmt};

use itertools::iproduct;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
    Ace,
    Numeric(u8),
    Jack,
    Queen,
    King,
    Joker,
}

impl Into<CardColor> for CardSuits {
    fn into(self) -> CardColor {
        match self {
            CardSuits::Clubs | CardSuits::Spades => CardColor::Black,
            CardSuits::Diamonds | CardSuits::Hearts => CardColor::Red,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CardColor {
    Red,
    Black,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum CardSuits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug)]
pub struct Card {
    pub card_type: CardType,
    pub color: CardColor,
    pub suit: CardSuits,
}

#[derive(Debug)]
struct InvalidCard {}

impl fmt::Display for InvalidCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid card value")
    }
}

impl error::Error for InvalidCard {}

impl Card {
    pub fn new(card_type: CardType, suit: CardSuits) -> Result<Card, Box<dyn error::Error>> {
        let color = match suit {
            CardSuits::Clubs | CardSuits::Spades => CardColor::Black,
            CardSuits::Diamonds | CardSuits::Hearts => CardColor::Red,
        };

        match card_type {
            CardType::Numeric(value) if value > 1 && value < 11 => Ok(Card {
                card_type,
                color,
                suit,
            }),
            CardType::Numeric(_) => Err(Box::new(InvalidCard {})),
            _ => Ok(Card {
                card_type,
                color,
                suit,
            }),
        }
    }
}

pub fn generate_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(54);

    for (card_type, suit) in iproduct!(CardType::iter(), CardSuits::iter()) {
        match card_type {
            CardType::Numeric(_) => (2..11).for_each(|value| {
                deck.push(Card::new(CardType::Numeric(value), suit).unwrap());
            }),
            CardType::Joker => {
                let joker_doesnt_exist = deck
                    .iter()
                    .find(|card| card.card_type == CardType::Joker && card.color == suit.into())
                    .is_none();

                if joker_doesnt_exist {
                    deck.push(Card::new(card_type, suit).unwrap());
                }
            }
            _ => {
                deck.push(Card::new(card_type, suit).unwrap());
            }
        }
    }

    deck
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    #[test]
    /// Se pueden crear las variantes distintas sin problemas.
    fn test_valid_variants() {
        assert!(Card::new(CardType::Ace, CardSuits::Spades).is_ok());
        assert!(Card::new(CardType::Jack, CardSuits::Hearts).is_ok());
        assert!(Card::new(CardType::Queen, CardSuits::Diamonds).is_ok());
        assert!(Card::new(CardType::King, CardSuits::Clubs).is_ok());
        assert!(Card::new(CardType::Joker, CardSuits::Spades).is_ok());
    }

    #[test]
    /// Se pueden crear cartas entre 2 y 10.
    fn test_numeric_variants() {
        for number in 2..11 {
            let suit = random_suit(random::<u32>());
            assert!(Card::new(CardType::Numeric(number), suit).is_ok())
        }
    }

    #[test]
    /// No se pueden crear variantes de valor fuera del rango.
    fn test_invalid_numeric_variants() {
        for i in 0..100 {
            let value: u8 = random();
            let suit = random_suit(i);

            if (2..11).contains(&value) {
                continue;
            }

            assert!(Card::new(CardType::Numeric(value as u8), suit).is_err());
        }
    }

    #[test]
    /// Deck is generated as expected.
    fn test_initial_deck_is_valid() {
        let deck = generate_deck();

        assert_eq!(deck.len(), 54);
        assert_eq!(deck.capacity(), 54);
        // TODO: Agregar verificar que las cartas sean únicas.
    }

    /// A partir de un numero, genera un `CardSuit`.
    fn random_suit(input: u32) -> CardSuits {
        match input % 4 {
            0 => CardSuits::Clubs,
            1 => CardSuits::Spades,
            2 => CardSuits::Hearts,
            _ => CardSuits::Diamonds,
        }
    }
}
