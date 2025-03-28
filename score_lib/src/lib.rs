mod card;

mod modifiers;
pub mod config;

pub use config::explain_enabled;
mod utils;
mod poker_hand;
use modifiers::EditionUtils;
use modifiers::EnhancementUtils;
use ortalib::Card;
use ortalib::JokerCard;

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
    best_poker_hand: PokerHand,
    poker_hands: Vec<PokerHand>,
    scored_cards: Vec<Card>,
    // cards_played: Vec<Card>,
    cards_held_in_hand: Vec<Card>,
    jokers: Vec<JokerCard>,
    raised_first_triggered: bool,
    photograph_triggered: bool,
}

impl Score {
    pub fn new(round: Round) -> Self {
        let poker_hands = PokerHand::recogonize(&round.cards_played);
        explain!("{:?}", poker_hands);
        let best_poker_hand: PokerHand = poker_hands
            .first()
            .copied()
            .unwrap_or(PokerHand::HighCard);
        let scored_cards = round.get_scored_cards(best_poker_hand);
        Score {
            mult: 0.0,
            chips: 0.0,
            poker_hands,
            // cards_played: round.cards_played,
            scored_cards,
            best_poker_hand,
            cards_held_in_hand: round.cards_held_in_hand,
            jokers: round.jokers,
            photograph_triggered: false,
            raised_first_triggered: false,
        }
    }
    pub fn calculate_score(&mut self) {
        // deal with cards played
        self.best_poker_hand.apply(&mut self.chips, &mut self.mult);

        explain!("{:?}", self.scored_cards);
        self.scored_cards
            .clone()
            .iter()
            .for_each(|card| {
                self.chips += card.rank.rank_value();
                explain!(
                    "{:?}{:?} +{:?} chips {:?}",
                    card.rank,
                    card.suit,
                    card.rank.rank_value(),
                    (self.chips, self.mult)
                );
                // modifiers(self.explain, vec![*card], &mut self.chips, &mut self.mult);
                card.enhancement.apply(self, false);
                card.edition.apply(self);
                self.jokers
                    .clone()
                    .iter()
                    .filter(|e| e.is_on_scored())
                    .for_each(|x| x.apply_on_scored(self, card));
            });
        // deal with cards held in hand
        explain!("\n----cards held in hand----");
        let cards = self.cards_held_in_hand.clone();
        let len = cards.len();

        for (i, card) in cards.into_iter().enumerate() {
            let is_last = i == len - 1;
            explain!("☛ {:?}", card);
            card.enhancement.apply(self, true);
            self.jokers
                .clone()
                .iter()
                .filter(|e| e.is_on_held())
                .for_each(|x| x.apply_on_held(self, &card, is_last));
        }
        // handle jokers
        explain!("\n---jokers----");
        self.jokers
            .clone()
            .iter()
            .filter(|e| e.is_dependent())
            .for_each(|x| x.apply_independent(self));
    }
}
