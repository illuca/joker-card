use std::collections::{ HashSet };

use crate::card::CardUtils;
use crate::utils::{ num_of_poker_by_suits };
use crate::{ explain, Score };
use crate::modifiers::EditionUtils;
use ortalib::{ Card, Chips, Joker, JokerCard, Mult, PokerHand, Rank, Suit };

// pub enum JokerType {
//     OnScored,
//     OnHeld,
//     Independent,
// }

pub trait JokerUtils {
    fn joker_value(&self, s: &Score) -> (Chips, Mult, String);
    fn meet_condition(&self, s: &mut Score) -> bool;
    fn is_on_scored(&self) -> bool;
    fn is_on_held(&self) -> bool;
    fn is_dependent(&self) -> bool;
    fn apply_independent(&self, s: &mut Score) -> ();
    fn apply_on_scored(&self, s: &mut Score, card: &Card) -> ();
    fn apply_on_held(&self, s: &mut Score, card: &Card, is_last: bool);
}

impl JokerUtils for JokerCard {
    fn is_on_held(&self) -> bool {
        matches!(self.joker, Joker::RaisedFist | Joker::Baron)
    }
    fn is_dependent(&self) -> bool {
        matches!(
            self.joker,
            Joker::Joker |
                Joker::JollyJoker |
                Joker::ZanyJoker |
                Joker::MadJoker |
                Joker::CrazyJoker |
                Joker::DrollJoker |
                Joker::SlyJoker |
                Joker::WilyJoker |
                Joker::CleverJoker |
                Joker::DeviousJoker |
                Joker::CraftyJoker |
                Joker::AbstractJoker |
                Joker::Blackboard |
                Joker::FlowerPot
        )
    }
    fn is_on_scored(&self) -> bool {
        matches!(
            self.joker,
            Joker::GreedyJoker |
                Joker::LustyJoker |
                Joker::WrathfulJoker |
                Joker::GluttonousJoker |
                Joker::Fibonacci |
                Joker::ScaryFace |
                Joker::EvenSteven |
                Joker::OddTodd |
                Joker::Photograph |
                Joker::SmileyFace
        )
    }
    fn apply_independent(&self, s: &mut Score) -> () {
        if !self.meet_condition(s) {
            explain!("🃏{:?}", &self);
            self.edition.apply(s);
            return;
        }

        match &self.joker {
            | Joker::Joker
            | Joker::JollyJoker
            | Joker::ZanyJoker
            | Joker::MadJoker
            | Joker::CrazyJoker
            | Joker::DrollJoker
            | Joker::SlyJoker
            | Joker::WilyJoker
            | Joker::CleverJoker
            | Joker::DeviousJoker
            | Joker::CraftyJoker
            | Joker::AbstractJoker
            | Joker::Blackboard
            | Joker::FlowerPot => {
                let (c, m, msg) = self.joker_value(s);
                s.chips = c;
                s.mult = m;
                explain!("🃏{:?} {} {:?}", &self, msg, (s.chips, s.mult));
            }
            _ => panic!("joker not found"),
        }
        self.edition.apply(s);
    }

    fn apply_on_scored(&self, s: &mut Score, card: &Card) {
        match &self.joker {
            Joker::GreedyJoker => {
                if card.suit == Suit::Diamonds || card.is_wild() {
                    s.mult += 3.0;
                    explain!("🃏 {:?} +3 Mult", &self);
                }
            }
            Joker::LustyJoker => {
                if card.suit == Suit::Hearts || card.is_wild() {
                    s.mult += 3.0;
                    explain!("🃏 {:?} +3 Mult", &self);
                }
            }
            Joker::WrathfulJoker => {
                if card.suit == Suit::Spades || card.is_wild() {
                    s.mult += 3.0;
                    explain!("🃏 {:?} +3 Mult", &self);
                }
            }
            Joker::GluttonousJoker => {
                if card.suit == Suit::Clubs || card.is_wild() {
                    s.mult += 3.0;
                    explain!("🃏 {:?} +3 Mult", &self);
                }
            }
            Joker::Fibonacci => {
                let fibonacci_ranks = [Rank::Ace, Rank::Two, Rank::Three, Rank::Five, Rank::Eight];
                if fibonacci_ranks.contains(&card.rank) {
                    s.mult += 8.0;
                    explain!("🃏 {:?} +8 Mult", &self);
                }
            }
            Joker::ScaryFace => {
                if card.rank.is_face() {
                    s.chips += 30.0;
                    explain!("🃏 {:?} +30 Chips", &self);
                }
            }
            Joker::EvenSteven => {
                let even_ranks = [Rank::Two, Rank::Four, Rank::Six, Rank::Eight, Rank::Ten];
                if even_ranks.contains(&card.rank) {
                    s.mult += 4.0;
                    explain!("🃏 {:?} +4 Mult", &self);
                }
            }
            Joker::OddTodd => {
                let odd_ranks = [Rank::Ace, Rank::Three, Rank::Five, Rank::Seven, Rank::Nine];
                if odd_ranks.contains(&card.rank) {
                    s.chips += 31.0;
                    explain!("🃏 {:?} +31 Chips", &self);
                }
            }
            Joker::Photograph => {
                if card.rank.is_face() && !s.photograph_triggered {
                    s.mult *= 2.0;
                    s.photograph_triggered = true;
                    explain!("🃏 {:?} x2 Mult", &self);
                }
            }
            Joker::SmileyFace => {
                if card.rank.is_face() {
                    s.mult += 5.0;
                    explain!("🃏 {:?} +5 Mult", &self);
                }
            }
            _ => todo!(), // 其他Joker类型
        }
    }
    fn apply_on_held(&self, s: &mut Score, card: &Card, is_last: bool) {
        match &self.joker {
            Joker::RaisedFist => {
                // clone and sort reversed, once joker triggered, won't trigger again
                if !s.raised_first_triggered {
                    if is_last {
                        s.mult += 2.0 * card.rank.rank_value();
                        s.raised_first_triggered = true;
                    }
                }
            }
            Joker::Baron => {
                if card.rank == Rank::King {
                    s.mult *= 1.5;
                    explain!("🃏 {:?} x1.5 Mult", &self);
                }
            }
            _ => panic!("on held joker not found."),
        }
    }

    fn meet_condition(&self, s: &mut Score) -> bool {
        match &self.joker {
            Joker::JollyJoker => s.poker_hands.contains(&PokerHand::Pair),
            Joker::ZanyJoker => s.poker_hands.contains(&PokerHand::ThreeOfAKind),
            Joker::MadJoker => s.poker_hands.contains(&PokerHand::TwoPair),
            Joker::CrazyJoker => s.poker_hands.contains(&PokerHand::Straight),
            Joker::DrollJoker => s.poker_hands.contains(&PokerHand::Flush),
            Joker::SlyJoker => s.poker_hands.contains(&PokerHand::Pair),
            Joker::WilyJoker => s.poker_hands.contains(&PokerHand::ThreeOfAKind),
            Joker::CleverJoker => s.poker_hands.contains(&PokerHand::TwoPair),
            Joker::DeviousJoker => s.poker_hands.contains(&PokerHand::Straight),
            Joker::CraftyJoker => s.poker_hands.contains(&PokerHand::Flush),
            Joker::Blackboard => {
                let n = num_of_poker_by_suits(
                    &s.cards_held_in_hand,
                    vec![Suit::Clubs, Suit::Spades]
                );
                return s.cards_held_in_hand.is_empty() || s.cards_held_in_hand.len() == n;
            }
            Joker::FlowerPot => {
                let small: HashSet<Suit> = s.scored_cards
                    .iter()
                    .filter(|x| !x.is_wild())
                    .map(|x| x.suit)
                    .collect();
                let big: HashSet<Suit> = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
                    .iter()
                    .cloned()
                    .collect();
                let diff: HashSet<Suit> = big.difference(&small).cloned().collect();

                let n_wild_cards = s.scored_cards
                    .iter()
                    .filter(|x| x.is_wild())
                    .count();
                return diff.len() <= n_wild_cards;
            }
            _ => true,
        }
    }
    fn joker_value(&self, s: &Score) -> (Chips, Mult, String) {
        match self.joker {
            Joker::Joker => (s.chips, s.mult + 4.0, "+4 Mult".to_string()),
            Joker::JollyJoker => (s.chips, s.mult + 8.0, "+8 Mult".to_string()),
            Joker::ZanyJoker => (s.chips, s.mult + 12.0, "+12 Mult".to_string()),
            Joker::MadJoker => (s.chips, s.mult + 10.0, "+10 Mult".to_string()),
            Joker::CrazyJoker => (s.chips, s.mult + 12.0, "+12 Mult".to_string()),
            Joker::DrollJoker => (s.chips, s.mult + 10.0, "+10 Mult".to_string()),
            Joker::SlyJoker => (s.chips + 50.0, s.mult, "+50 Chips".to_string()),
            Joker::WilyJoker => (s.chips + 100.0, s.mult, "+100 Chips".to_string()),
            Joker::CleverJoker => (s.chips + 80.0, s.mult, "+80 Chips".to_string()),
            Joker::DeviousJoker => (s.chips + 100.0, s.mult, "+100 Chips".to_string()),
            Joker::CraftyJoker => (s.chips + 80.0, s.mult, "+80 Chips".to_string()),
            Joker::AbstractJoker => {
                let n = s.jokers.len() as f64;
                return (s.chips, s.mult + 3.0 * n, format!("+3x{n} Mult"));
            }
            Joker::Blackboard => (s.chips, s.mult * 3.0, "x3 Mult".to_string()),
            Joker::FlowerPot => (s.chips, s.mult * 3.0, "x3 Mult".to_string()),
            _ => panic!("Joker type not found"),
        }
    }
}
