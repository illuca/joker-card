use std::ops::{Bound, Mul};

use itertools::sorted;
use ortalib::{Card, Chips, Edition, Enhancement, Mult, PokerHand, Rank, Round};

pub struct Score {
    pub mult: Mult,
    pub chips: Chips,
}

impl Score {
    pub fn new() -> Self {
        Score {
            mult: 0.0,
            chips: 0.0,
        }
    }
    pub fn calculate_score(&mut self, round: &Round) {
        let poker_hand = round.recognize_hand_type();
        let scored_cards = round.get_scored_cards(poker_hand);
        scored_cards.iter().for_each(|card| {
            self.chips += card.rank.rank_value();
        });
        self.modifiers(scored_cards);
    }
    fn modifiers(&mut self, scored_cards: Vec<Card>) {
        for card in scored_cards {
            // modifiers
            let (c1, m1) = match card.edition {
                Some(Edition::Foil) => (50.0, 0.0),
                Some(Edition::Holographic) => (0.0, 10.0),
                Some(Edition::Polychrome) => (0.0, 1.5),
                None => (0.0, 0.0),
            };
            self.chips += c1;
            self.mult += m1;
            // enhancement
            let (c2, m2) = card
                .enhancement
                .map(|e| e.get_enhancement_value(false))
                .unwrap_or((0.0, 0.0));
            self.chips += c2;
            self.mult += m2;
        }
    }
}

pub trait EnhancementValueGetter {
    fn get_enhancement_value(&self, is_held_in_hand: bool) -> (Chips, Mult);
}
impl EnhancementValueGetter for Enhancement {
    fn get_enhancement_value(&self, is_held_in_hand: bool) -> (Chips, Mult) {
        if is_held_in_hand {
            match *self {
                Enhancement::Bonus => (50.0, 0.0),
                Enhancement::Glass => (0.0, 10.0),
                Enhancement::Mult => (0.0, 1.5),
                _ => (0.0, 0.0),
            }
        } else {
            match *self {
                Enhancement::Steel => (0.0, 1.5),
                _ => (0.0, 0.0),
            }
        }
    }
}

pub trait ScoredCardGetter {
    fn get_scored_cards(&self, poker_hand: PokerHand) -> Vec<Card>;
}
impl ScoredCardGetter for Round {
    fn get_scored_cards(&self, poker_hand: PokerHand) -> Vec<Card> {
        // todo!();
        return self.cards_played.clone();
    }
}

pub trait HandPokerRecognizer {
    fn recognize_hand_type(&self) -> PokerHand;
    fn is_flush(cards_played: &Vec<Card>) -> bool;
    fn is_straight(cards_played: &Vec<Card>) -> bool;
}

impl HandPokerRecognizer for Round {
    fn recognize_hand_type(&self) -> PokerHand {
        if Self::is_straight(&self.cards_played) {
            return PokerHand::Straight;
        } else if Self::is_flush(&self.cards_played) {
            return PokerHand::Flush;
        }
        todo!()
    }

    fn is_straight(cards_played: &Vec<Card>) -> bool {
        if cards_played.len() < 5 {
            return false;
        }

        let mut cards = cards_played.clone();

        cards.sort_by_key(|card| card.rank.straight_value());

        let mut is_consecutive = true;
        for i in 0..cards.len() - 1 {
            if cards[i + 1].rank.straight_value() != cards[i].rank.straight_value() + 1 {
                is_consecutive = false;
                break;
            }
        }

        if is_consecutive {
            return true;
        }

        let has_ace = cards.iter().any(|card| matches!(card.rank, Rank::Ace));
        let has_two = cards.iter().any(|card| matches!(card.rank, Rank::Two));
        let has_three = cards.iter().any(|card| matches!(card.rank, Rank::Three));
        let has_four = cards.iter().any(|card| matches!(card.rank, Rank::Four));
        let has_five = cards.iter().any(|card| matches!(card.rank, Rank::Five));

        has_ace && has_two && has_three && has_four && has_five
    }

    fn is_flush(cards_played: &Vec<Card>) -> bool {
        if cards_played.len() < 5 {
            return false;
        }
        let suit = cards_played[0].suit;

        for card in cards_played {
            let is_wild = match card.enhancement {
                Some(e) => e == Enhancement::Wild,
                None => false,
            };

            if card.suit == suit || is_wild {
                continue;
            } else {
                return false;
            }
        }
        return true;
    }
}
trait StraghtValueGetter {
    fn straight_value(&self) -> u8;
}
impl StraghtValueGetter for Rank {
    fn straight_value(&self) -> u8 {
        match *self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14, // it can be 14 or 1
        }
    }
}
