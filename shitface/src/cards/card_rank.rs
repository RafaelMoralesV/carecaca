use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use strum::EnumIter;

/// Represents the rank of a card.
#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardRank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,

    Jack,
    Queen,
    King,
    Joker,
}

impl Distribution<CardRank> for Standard {
    /// Generates a random card rank.
    ///
    /// # Arguments
    ///
    /// * `rng` - The random number generator.
    ///
    /// # Returns
    ///
    /// A randomly generated card rank.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardRank {
        match rng.gen_range(0..=13) {
            0 => CardRank::Ace,
            1 => CardRank::Two,
            2 => CardRank::Three,
            3 => CardRank::Four,
            4 => CardRank::Five,
            5 => CardRank::Six,
            6 => CardRank::Seven,
            7 => CardRank::Eight,
            8 => CardRank::Nine,
            9 => CardRank::Ten,
            10 => CardRank::Jack,
            11 => CardRank::Queen,
            12 => CardRank::King,
            _ => CardRank::Joker,
        }
    }
}
