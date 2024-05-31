use std::collections::VecDeque;

use itertools::iproduct;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

    pub fn shuffled_new() -> Self {
        let mut deck = Self::new();
        deck.shuffle();

        deck
    }

    pub fn cards_left(&self) -> usize {
        self.0.len()
    }

    pub fn shuffle(&mut self) {
        self.0.make_contiguous().shuffle(&mut thread_rng());
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
    use crate::deck::Deck;

    #[test]
    /// Deck is generated as expected.
    fn test_initial_deck_is_valid() {
        let deck = Deck::new();

        assert_eq!(deck.cards_left(), 54, "Deck length isn't 54, somehow.");
        // TODO: Agregar verificar que las cartas sean únicas.
    }
}
