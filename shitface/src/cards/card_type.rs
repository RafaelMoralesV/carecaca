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

#[cfg(test)]
mod tests {
    use rand::random;

    use super::*;

    #[test]
    fn test_joker_is_the_highest() {
        let joker = CardType::Joker;

        for _ in 0..100 {
            let other: CardType = random();

            if other == CardType::Joker {
                assert_eq!(joker, other);
            } else {
                assert!(joker > other, "Joker is not greater than {other:?}")
            }
        }
    }

    #[test]
    fn test_ace_is_the_second_highest() {
        let ace = CardType::Ace;

        for _ in 0..100 {
            let other: CardType = random();

            match other {
                CardType::Joker => assert!(other > ace, "Ace isn't Less than Joker."),
                CardType::Ace => assert_eq!(other, ace, "Ace isn't equal to itself."),
                _ => assert!(ace > other, "Ace isn't greater than {other:?}."),
            }
        }
    }

    #[test]
    fn test_king_ordering() {
        let king = CardType::King;

        for _ in 0..100 {
            let other: CardType = random();

            use CardType::*;
            match other {
                Joker | Ace => assert!(other > king, "King isn't less than {other:?}"),
                King => assert_eq!(other, king, "King isn't equal to itself."),
                _ => assert!(king > other, "King isn't greater than {other:?}."),
            }
        }
    }

    #[test]
    fn test_queen_ordering() {
        let queen = CardType::Queen;

        for _ in 0..100 {
            let other: CardType = random();

            use CardType::*;
            match other {
                Joker | Ace | King => assert!(other > queen, "Queen isn't less than {other:?}"),
                Queen => assert_eq!(other, queen, "Queen isn't equal to itself."),
                _ => assert!(queen > other, "Queen isn't greater than {other:?}."),
            }
        }
    }

    #[test]
    fn test_jack_ordering() {
        let jack = CardType::Jack;

        for _ in 0..100 {
            let other: CardType = random();

            use CardType::*;
            match other {
                Numeric(_) => assert!(other < jack, "Jack isn't Greater than {other:?}"),
                Jack => assert_eq!(other, jack, "Jack isn't equal to itself."),
                _ => assert!(jack < other, "Jack isn't Less than {other:?}."),
            }
        }
    }

    #[test]
    fn test_numeric_ordering() {
        let inner = random::<u8>() % 10 + 1;
        let numeric = CardType::Numeric(inner.clone());

        for _ in 0..100 {
            let other: CardType = random();

            use CardType::*;
            match other {
                Numeric(value) if inner > value => {
                    assert!(other < numeric, "{numeric:?} isn't Greater than {other:?}")
                }
                Numeric(value) if inner == value => {
                    assert!(other == numeric, "{numeric:?} isn't Equal than {other:?}")
                }
                Numeric(value) if inner < value => {
                    assert!(other > numeric, "{numeric:?} isn't Less than {other:?}")
                }
                _ => assert!(numeric < other, "{numeric:?} isn't Less than {other:?}."),
            }
        }
    }
}
