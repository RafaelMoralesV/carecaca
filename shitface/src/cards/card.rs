use super::{card_color::CardColor, card_rank::CardRank, card_suits::CardSuits};
use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Represents a playing card.
///
/// A card must have a Rank and a Suit. The color of the card is derived from it.
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
    pub fn new(rank: CardRank, suit: CardSuits) -> Result<Self> {
        match rank {
            CardRank::Numeric(value) if !(2..=10).contains(&value) => {
                Err(anyhow!("This card is Invalid"))
            }
            _ => Ok(Self {
                rank,
                color: suit.into(),
                suit,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{random, Rng};
    use rand_seeder::{Seeder, SipRng};

    #[test]
    /// Se pueden crear las variantes distintas sin problemas.
    fn test_valid_variants() {
        assert!(Card::new(CardRank::Ace, CardSuits::Spades).is_ok());
        assert!(Card::new(CardRank::Jack, CardSuits::Hearts).is_ok());
        assert!(Card::new(CardRank::Queen, CardSuits::Diamonds).is_ok());
        assert!(Card::new(CardRank::King, CardSuits::Clubs).is_ok());
        assert!(Card::new(CardRank::Joker, CardSuits::Spades).is_ok());
    }

    #[test]
    /// Se pueden crear cartas entre 2 y 10.
    fn test_numeric_variants() {
        for number in 2..11 {
            let suit = random_suit(random::<u32>());
            assert!(Card::new(CardRank::Numeric(number), suit).is_ok())
        }
    }

    #[test]
    /// No se pueden crear variantes de valor fuera del rango.
    fn test_invalid_numeric_variants() {
        let mut rng: SipRng = Seeder::from("Ni buena suerte en el juego (8)").make_rng();
        for i in 0..100 {
            let value: u8 = rng.gen();
            let suit = random_suit(i);

            if (2..11).contains(&value) {
                continue;
            }

            assert!(
                Card::new(CardRank::Numeric(value), suit).is_err(),
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
