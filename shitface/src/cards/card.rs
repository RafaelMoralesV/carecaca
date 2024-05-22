use super::{card_color::CardColor, card_suits::CardSuits, card_type::CardType};
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Card {
    pub card_type: CardType,
    pub color: CardColor,
    pub suit: CardSuits,
}

impl Card {
    pub fn new(card_type: CardType, suit: CardSuits) -> Result<Card> {
        match card_type {
            CardType::Numeric(value) if value < 2 || value > 10 => {
                Err(anyhow!("This card is Invalid"))
            }
            _ => Ok(Card {
                card_type,
                color: suit.into(),
                suit,
            }),
        }
    }
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

            assert!(
                Card::new(CardType::Numeric(value), suit).is_err(),
                "The card with numeric value [{value}] should error."
            );
        }
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
