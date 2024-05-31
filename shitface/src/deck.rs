use std::collections::VecDeque;

use itertools::iproduct;
use rand::prelude::*;

use crate::cards::{card::Card, card_rank::CardRank, card_suits::CardSuits};

pub struct Deck(VecDeque<Card>);

impl Deck {
    pub fn new() -> Self {
        use strum::IntoEnumIterator;

        let mut deck = VecDeque::with_capacity(54);

        for (card_type, suit) in iproduct!(CardRank::iter(), CardSuits::iter()) {
            match card_type {
                CardRank::Numeric(_) => (2..11).for_each(|value| {
                    let card = Card::new(CardRank::Numeric(value), suit);

                    match card {
                        Ok(c) => deck.push_back(c),
                        Err(_) => unreachable!(),
                    }
                }),
                CardRank::Joker => {
                    let joker_doesnt_exist = !deck
                        .iter()
                        .any(|card| card.rank == CardRank::Joker && card.color == suit.into());

                    if joker_doesnt_exist {
                        let card = Card::new(card_type, suit);

                        match card {
                            Ok(c) => deck.push_back(c),
                            Err(_) => unreachable!(),
                        }
                    }
                }
                _ => {
                    let card = Card::new(card_type, suit);

                    match card {
                        Ok(c) => deck.push_back(c),
                        Err(_) => unreachable!(),
                    }
                }
            }
        }

        Self(deck)
    }

    pub fn shuffled_new<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        let mut deck = Self::new();
        deck.shuffle(rng);

        deck
    }

    pub fn cards_left(&self) -> usize {
        self.0.len()
    }

    pub fn shuffle<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        self.0.make_contiguous().shuffle(rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop_back()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use rand_seeder::{Seeder, SipRng};

    use crate::deck::Deck;

    #[test]
    /// Deck is generated as expected.
    fn test_initial_deck_is_valid() {
        let deck = Deck::new();

        assert_eq!(deck.cards_left(), 54, "Deck length isn't 54, somehow.");
    }

    #[test]
    fn test_deck_has_no_repeated_cards() {
        let mut deck = Deck::new().0;

        // Orderno el mazo.
        deck.make_contiguous().sort();

        // Convierto de VecDeque a Vec
        let mut deck: Vec<_> = deck.into();

        // Quito duplicados.
        deck.dedup();

        assert_eq!(deck.len(), 54, "Deck had duplicate values");
    }

    #[test]
    fn test_shuffling_works() {
        let mut rng: SipRng = Seeder::from("Barajale barajale wn oh").make_rng();

        let mut shuffled_deck = Deck::shuffled_new(&mut rng);
        let mut unshuffled_deck = Deck::new();

        shuffled_deck.shuffle(&mut rng);

        // Normalmente esto podria ser peligroso, pero con el seeder declarado, se que estos dos
        // mazos nunca son iguales.
        while shuffled_deck.cards_left() > 0 {
            assert_ne!(
                shuffled_deck.draw(),
                unshuffled_deck.draw(),
                "La mezcla del mazo fallo"
            );
        }
    }
}
