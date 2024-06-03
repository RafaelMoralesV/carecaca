//! # Carecaca Library
//!
//! This library provides functionality to simulate the game of Carecaca, a card game played with a standard 52-card
//! deck plus jokers.
//!
//! ## Rules
//!
//! - Each player starts with 3 cards in hand, 3 cards face down on the table, and 3 cards face up on the table.
//! - Cards on the table are not touched until the player has no cards in hand. When a card is played, the players
//!     draws from the deck until having at least 3 cards in hand.
//! - The player who starts can play any card. Subsequent players must play a card of equal or higher rank. If unable
//!     to do so, they take the pile of accumulated cards.
//! - If four identical cards are played consecutively, they are removed from the game, and the player who played the
//!     fourth card goes again.
//!
//! Exceptions:
//! - Ace is the highest card.
//! - Two is a wildcard and can be played on any card.
//! - After a seven, a card of equal or lower rank must be played.
//! - Ten burns the pile of accumulated cards.
//! - Jack reverses the order of turns.
//! - Joker makes the next player take the accumulated pile, unless that player also plays a Joker.
//!
//! When the deck and hand are empty, players use cards face up on the table. When those are gone, players use
//! face-down cards randomly.
//!
//! If a player runs out of cards, they exit the game. The last player with cards is the Carecaca.

pub mod cards;
pub mod deck;
pub mod player;
