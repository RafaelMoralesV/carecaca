use std::{error, fmt};

#[derive(Debug)]
pub enum CardType {
    Ace,
    Numeric(u8),
    Jack,
    Queen,
    King,
    Joker,
}

#[derive(Debug)]
pub enum CardColor {
    Red,
    Black,
}

#[derive(Debug)]
pub enum CardSuits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug)]
pub struct Card {
    card_type: CardType,
    color: CardColor,
    suit: CardSuits,
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
