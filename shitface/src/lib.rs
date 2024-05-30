use cards::{card::Card, card_rank::CardRank, card_suits::CardSuits};
use itertools::iproduct;

mod cards;

#[must_use]
pub fn generate_deck() -> Vec<Card> {
    use strum::IntoEnumIterator;

    let mut deck: Vec<Card> = Vec::with_capacity(54);

    for (card_type, suit) in iproduct!(CardRank::iter(), CardSuits::iter()) {
        match card_type {
            CardRank::Numeric(_) => (2..11).for_each(|value| {
                let card = Card::new(CardRank::Numeric(value), suit);

                match card {
                    Ok(c) => deck.push(c),
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
                        Ok(c) => deck.push(c),
                        Err(_) => unreachable!(),
                    }
                }
            }
            _ => {
                let card = Card::new(card_type, suit);

                match card {
                    Ok(c) => deck.push(c),
                    Err(_) => unreachable!(),
                }
            }
        }
    }

    deck
}

#[cfg(test)]
mod tests {
    use crate::generate_deck;

    #[test]
    /// Deck is generated as expected.
    fn test_initial_deck_is_valid() {
        let deck = generate_deck();

        assert_eq!(deck.len(), 54, "Deck length isn't 54, somehow.");
        assert_eq!(deck.capacity(), 54, "Deck capacity grew somehow.");
        // TODO: Agregar verificar que las cartas sean únicas.
    }
}
