use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use strum::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Ord)]
pub enum CardType {
    Ace,
    Numeric(u8),
    Jack,
    Queen,
    King,
    Joker,
}

impl PartialOrd for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        if self.eq(&other) {
            return Some(Ordering::Equal);
        }

        use CardType::*;
        match self {
            Ace => match other {
                Joker => Some(Ordering::Less),
                _ => Some(Ordering::Greater),
            },
            Numeric(value) => match other {
                Numeric(other_value) => value.partial_cmp(&other_value),
                _ => Some(Ordering::Less),
            },
            Jack => match other {
                Numeric(_) => Some(Ordering::Greater),
                _ => Some(Ordering::Less),
            },
            Queen => match other {
                Ace | King | Joker => Some(Ordering::Less),
                _ => Some(Ordering::Greater),
            },
            King => match other {
                Ace | Joker => Some(Ordering::Less),
                _ => Some(Ordering::Greater),
            },
            Joker => Some(Ordering::Greater),
        }
    }
}

impl Distribution<CardType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardType {
        match rng.gen_range(0..=5) {
            0 => CardType::Ace,
            1 => CardType::Numeric(rng.gen_range(2..=10)),
            2 => CardType::Jack,
            3 => CardType::Queen,
            4 => CardType::King,
            _ => CardType::Joker,
        }
    }
}

