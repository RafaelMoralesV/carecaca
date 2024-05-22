use cards::{card::Card, card_suits::CardSuits, card_type::CardType};
use itertools::iproduct;

mod cards;

pub fn generate_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(54);

    use strum::IntoEnumIterator;
    for (card_type, suit) in iproduct!(CardType::iter(), CardSuits::iter()) {
        match card_type {
            CardType::Numeric(_) => (2..11).for_each(|value| {
                deck.push(Card::new(CardType::Numeric(value), suit).unwrap());
            }),
            CardType::Joker => {
                let joker_doesnt_exist = deck
                    .iter()
                    .find(|card| card.card_type == CardType::Joker && card.color == suit.into())
                    .is_none();

                if joker_doesnt_exist {
                    deck.push(Card::new(card_type, suit).unwrap());
                }
            }
            _ => {
                deck.push(Card::new(card_type, suit).unwrap());
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

        assert_eq!(deck.len(), 54);
        assert_eq!(deck.capacity(), 54);
        // TODO: Agregar verificar que las cartas sean únicas.
    }
}
