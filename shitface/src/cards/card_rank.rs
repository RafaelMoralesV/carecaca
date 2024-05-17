use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use strum::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum CardRank {
    Ace,
    Numeric(u8),
    Jack,
    Queen,
    King,
    Joker,
}

impl PartialOrd for CardRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        use CardRank::{Ace, Jack, Joker, King, Numeric, Queen};

        if self.eq(other) {
            return Ordering::Equal;
        }

        match self {
            Ace => match other {
                Joker => Ordering::Less,
                _ => Ordering::Greater,
            },
            Numeric(value) => match other {
                Numeric(other_value) => value.cmp(other_value),
                _ => Ordering::Less,
            },
            Jack => match other {
                Numeric(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            Queen => match other {
                Ace | King | Joker => Ordering::Less,
                _ => Ordering::Greater,
            },
            King => match other {
                Ace | Joker => Ordering::Less,
                _ => Ordering::Greater,
            },
            Joker => Ordering::Greater,
        }
    }
}

impl Distribution<CardRank> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardRank {
        match rng.gen_range(0..=5) {
            0 => CardRank::Ace,
            1 => CardRank::Numeric(rng.gen_range(2..=10)),
            2 => CardRank::Jack,
            3 => CardRank::Queen,
            4 => CardRank::King,
            _ => CardRank::Joker,
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::random;

    use super::*;

    #[test]
    fn test_joker_is_the_highest() {
        let joker = CardRank::Joker;

        for _ in 0..100 {
            let other: CardRank = random();

            if other == CardRank::Joker {
                assert_eq!(joker, other);
            } else {
                assert!(joker > other, "Joker is not greater than {other:?}")
            }
        }
    }

    #[test]
    fn test_ace_is_the_second_highest() {
        let ace = CardRank::Ace;

        for _ in 0..100 {
            let other: CardRank = random();

            match other {
                CardRank::Joker => assert!(other > ace, "Ace isn't Less than Joker."),
                CardRank::Ace => assert_eq!(other, ace, "Ace isn't equal to itself."),
                _ => assert!(ace > other, "Ace isn't greater than {other:?}."),
            }
        }
    }

    #[test]
    fn test_king_ordering() {
        let king = CardRank::King;

        for _ in 0..100 {
            let other: CardRank = random();

            use CardRank::*;
            match other {
                Joker | Ace => assert!(other > king, "King isn't less than {other:?}"),
                King => assert_eq!(other, king, "King isn't equal to itself."),
                _ => assert!(king > other, "King isn't greater than {other:?}."),
            }
        }
    }

    #[test]
    fn test_queen_ordering() {
        let queen = CardRank::Queen;

        for _ in 0..100 {
            let other: CardRank = random();

            use CardRank::*;
            match other {
                Joker | Ace | King => assert!(other > queen, "Queen isn't less than {other:?}"),
                Queen => assert_eq!(other, queen, "Queen isn't equal to itself."),
                _ => assert!(queen > other, "Queen isn't greater than {other:?}."),
            }
        }
    }

    #[test]
    fn test_jack_ordering() {
        let jack = CardRank::Jack;

        for _ in 0..100 {
            let other: CardRank = random();

            use CardRank::*;
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
        let numeric = CardRank::Numeric(inner);

        for _ in 0..100 {
            let other: CardRank = random();

            use CardRank::*;
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
