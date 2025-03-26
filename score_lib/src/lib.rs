mod modifiers;
pub mod config;
pub use config::explain_enabled;
mod utils;
mod poker_hand;
use modifiers::EditionUtils;
use modifiers::EnhancementUtils;
pub use poker_hand::HandPokerRecognizer;

mod getters;
pub use getters::EnhancementValueGetter;
pub use getters::ScoredCardGetter;
pub use getters::StraghtValueGetter;
mod jokers;
pub use jokers::JokerUtils;
use ortalib::{ Chips, Mult, PokerHand, Round };
use poker_hand::PokerHandUtils;

pub struct Score {
    pub mult: Mult,
    pub chips: Chips,
    poker_hand: PokerHand,
    round: Round,
}

impl Score {
    pub fn new(r: Round) -> Self {
        Score {
            mult: 0.0,
            chips: 0.0,
            poker_hand: PokerHand::HighCard,
            round: r,
        }
    }
    pub fn calculate_score(&mut self) {
        // deal with cards played

        self.poker_hand = PokerHand::recogonize(&self.round);
        self.poker_hand.apply(&mut self.chips, &mut self.mult);

        let scored_cards = &self.round.get_scored_cards(self.poker_hand);
        explain!("{:?}", scored_cards);
        scored_cards.iter().for_each(|card| {
            self.chips += card.rank.rank_value();
            explain!("{:?}{:?} +{:?} chips {:?}", card.rank, card.suit, card.rank.rank_value(), (
                self.chips,
                self.mult,
            ));
            // modifiers(self.explain, vec![*card], &mut self.chips, &mut self.mult);
            card.enhancement.apply(self, false);
            card.edition.apply(self);
        });
        // deal with cards held in hand
        explain!("\n----cards held in hand----");
        for card in self.round.cards_held_in_hand.clone() {
            explain!("☛ {:?}", card);
            card.enhancement.apply(self, true);
        }
        // handle jokers
        explain!("\n---jokers----");
        for joker_card in self.round.jokers.clone() {
            joker_card.apply(self);
        }
    }
}
