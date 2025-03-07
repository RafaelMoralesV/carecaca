//! Deck module.
//!
//! A deck is a collection of cards. This implementation uses a VecDeque mostly as a Stack of
//! cards. This stack is hidden away as implementation detail to users of this library tho, and
//! every way of interacting with the collection is hidden away inside de trait implementation.

use std::collections::VecDeque;

use itertools::Itertools;
use rand::prelude::*;

use crate::cards::{card::Card, card_rank::CardRank, card_suits::CardSuits};

/// A deck of cards.
///
/// This struct represents a 54-card deck, including the two Jokers, of a standard English deck.
/// There are two ways of creating a deck: using the `new()` constructor, which initializes the
/// deck in a predetermined order, and using the `shuffled_new()` constructor, which creates
/// a deck with cards shuffled randomly. Additionally, a default implementation is provided,
/// equivalent to calling `Deck::new()`.
pub struct Deck(VecDeque<Card>);

impl Deck {
    /// Creates a new deck.
    ///
    /// This function initializes a new deck of cards. However, it's important to note
    /// that the ordering of cards in this deck may not follow the standard conventions.
    /// Each time you create a new deck using this function, it will produce the same
    /// order of cards. The ordering is deterministic and not based on any randomness.
    ///
    /// # Examples
    ///
    /// ```
    /// use shitface::deck::Deck;
    ///
    /// let deck = Deck::new(); // `deck` is now initialized with cards in a predetermined order.
    ///
    /// assert_eq!(deck.cards_left(), 54);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `Deck` instance with cards initialized in a predetermined order.
    pub fn new() -> Self {
        use strum::IntoEnumIterator;

        let deck = CardRank::iter()
            .cartesian_product(CardSuits::iter())
            .map(|(rank, suit)| Card::new(rank, suit))
            .collect();

        Self(deck)
    }

    /// Creates a shuffled deck.
    ///
    /// Just like the `new` function, this creates a standard 54 cards deck. Implementation isn't
    /// speed optimal, but unless you're instantiating 100,000 decks, speed shouldn't be a problem,
    /// since we're only managing 54 cards.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator that
    ///          implements the `Rng` trait from the `rand` crate.
    ///
    /// # Example
    ///
    /// ```
    /// use rand::prelude::*;
    /// use shitface::deck::Deck;
    ///
    /// let deck = Deck::shuffled_new(&mut thread_rng()); // now `deck` is shuffled and ready.
    /// ```
    ///
    /// # Returns
    ///
    /// A new `Deck` instance, with cards shuffled.
    pub fn shuffled_new<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        let mut deck = Self::new();
        deck.shuffle(rng);

        deck
    }

    /// Indicates the number of cards remaining in the deck.
    ///
    /// This method returns the count of cards that are currently
    /// present in the deck. It accesses the internal representation
    /// of the deck, which is stored as a vector, and returns the
    /// length of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use shitface::deck::Deck;
    ///
    /// let mut deck = Deck::new();
    /// assert_eq!(deck.cards_left(), 54);
    /// for _ in (0..5) {
    ///     deck.draw(); // draw 5 cards
    /// }
    /// assert_eq!(deck.cards_left(), 49);
    /// ```
    pub fn cards_left(&self) -> usize {
        self.0.len()
    }

    /// Shuffles the deck using the provided random number generator.
    ///
    /// This method shuffles the deck using the Fisher-Yates algorithm,
    /// which is an efficient and unbiased way to shuffle a collection.
    /// It rearranges the order of cards in the deck randomly, ensuring
    /// that each permutation is equally likely.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator that
    ///          implements the `Rng` trait from the `rand` crate.
    ///
    /// # Examples
    ///
    /// ```
    /// use shitface::deck::Deck;
    /// use rand::thread_rng;
    ///
    /// let mut deck = Deck::new();
    /// deck.shuffle(&mut thread_rng()); // Now the deck is shuffled and ready for dealing.
    /// ```
    pub fn shuffle<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        self.0.make_contiguous().shuffle(rng);
    }

    /// Draws a card from the top of the deck.
    ///
    /// This method removes and returns the top card from the deck. If the deck
    /// is empty, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use shitface::deck::Deck;
    ///
    /// let mut deck = Deck::new();
    /// let card = deck.draw(); // `card` contains the top card from the deck, or `None` if the deck
    ///                         // is empty.
    /// ```
    ///
    /// # Returns
    ///
    /// - `Some(Card)`: The top card from the deck, if the deck is not empty.
    /// - `None`: If the deck is empty.
    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop_back()
    }
}

impl Default for Deck {
    /// Creates a default deck.
    ///
    /// This implementation returns a new deck initialized with cards in a predetermined order.
    /// It's equivalent to calling `Deck::new()`.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::Deck;
    use rand_seeder::{Seeder, SipRng};

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
