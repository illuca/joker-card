use crate::{ explain, Score };
use crate::modifiers::EditionUtils;
use crate::poker_hand::{ PokerHandUtils };
use ortalib::{ Chips, Joker, JokerCard, Mult, PokerHand };

pub trait JokerUtils {
    fn joker_value(&self, mult: Mult, num_jokers: usize) -> (Chips, Mult, String);
    fn meet_condition(&self, poker_hand: PokerHand) -> bool;
    fn apply(&self, s: &mut Score) -> ();
}

impl JokerUtils for JokerCard {
    fn apply(&self, s: &mut Score) -> () {
        if !self.meet_condition(s.poker_hand) {
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
            | Joker::AbstractJoker => {
                let (c, m, msg) = self.joker_value(s.mult, s.round.jokers.len());
                s.chips += c;
                s.mult += m;
                explain!("🃏{:?} {} {:?}", &self, msg, (s.chips, s.mult));
            }
            _ => todo!(),
        }
        self.edition.apply(s);
    }
    fn meet_condition(&self, poker_hand: PokerHand) -> bool {
        match &self.joker {
            Joker::JollyJoker => {
                return poker_hand.contains(PokerHand::Pair);
            }
            Joker::ZanyJoker => {
                return poker_hand.contains(PokerHand::ThreeOfAKind);
            }
            Joker::MadJoker => {
                return poker_hand.contains(PokerHand::TwoPair);
            }
            Joker::CrazyJoker => {
                return poker_hand.contains(PokerHand::Straight);
            }
            Joker::DrollJoker => {
                return poker_hand.contains(PokerHand::Flush);
            }
            Joker::SlyJoker => {
                return poker_hand.contains(PokerHand::Pair);
            }
            Joker::WilyJoker => {
                return poker_hand.contains(PokerHand::ThreeOfAKind);
            }
            Joker::CleverJoker => {
                return poker_hand.contains(PokerHand::TwoPair);
            }
            Joker::DeviousJoker => {
                return poker_hand.contains(PokerHand::Straight);
            }
            Joker::CraftyJoker => {
                return poker_hand.contains(PokerHand::Flush);
            }
            Joker::AbstractJoker => {
                return true;
            }
            _ => true,
        }
    }
    fn joker_value(&self, _: Mult, num_jokers: usize) -> (Chips, Mult, String) {
        match self.joker {
            Joker::Joker => (0.0, 4.0, "+4 Mult".to_string()),
            Joker::JollyJoker => (0.0, 8.0, "+8 Mult".to_string()),
            Joker::ZanyJoker => (0.0, 12.0, "+12 Mult".to_string()),
            Joker::MadJoker => (0.0, 10.0, "+10 Mult".to_string()),
            Joker::CrazyJoker => (0.0, 12.0, "+12 Mult".to_string()),
            Joker::DrollJoker => (0.0, 10.0, "+10 Mult".to_string()),
            Joker::SlyJoker => (50.0, 0.0, "+50 Chips".to_string()),
            Joker::WilyJoker => (100.0, 0.0, "+100 Chips".to_string()),
            Joker::CleverJoker => (80.0, 0.0, "+80 Chips".to_string()),
            Joker::DeviousJoker => (100.0, 0.0, "+100 Chips".to_string()),
            Joker::CraftyJoker => (80.0, 0.0, "+80 Chips".to_string()),
            Joker::AbstractJoker => {
                let adder = (num_jokers as f64) * 3.0;
                return (0.0, adder, format!("+ {:.1} Mult", adder));
            }
            _ => todo!(),
        }
    }
}
