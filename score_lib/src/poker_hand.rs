use crate::{
    explain,
    getters::StraghtValueGetter,
    utils::{ count_poker_by_rank, max_num_of_rank },
};

use ortalib::{ Card, Chips, Enhancement, Mult, PokerHand, Rank, Suit };
use std::{ collections::{ HashSet }, vec };

/// Checks if all cards in the played hand are of the same suit
/// Returns true if all non-wild cards are of the same suit or if all cards are wild
fn same_suite(cards_played: &Vec<Card>) -> bool {
    let mut suit_set: HashSet<Suit> = HashSet::new();
    for card in cards_played {
        match card.enhancement {
            // Ignore wild cards when checking for flush
            Some(Enhancement::Wild) => (),
            _ => {
                suit_set.insert(card.suit);
            }
        }
    }
    // check if we have less than 1 suit (can be 0 if all cards are wild)
    suit_set.len() < 1
}

/// Trait defining utility methods for poker hand evaluation
pub trait PokerHandUtils {
    /// Checks if the given cards match this poker hand
    fn is(&self, cards_played: &Vec<Card>) -> bool;

    /// Recognizes all possible poker hands from the given cards
    fn recogonize(cards_played: &Vec<Card>) -> Vec<PokerHand>;

    /// Returns a vector of all possible poker hands in descending order of value
    fn vector() -> Vec<PokerHand>;

    /// Applies the poker hand's value to the chips and multiplier
    fn apply(&self, chips: &mut Chips, mult: &mut Mult) -> ();
}

impl PokerHandUtils for PokerHand {
    /// Apply the hand's value to chips and multiplier
    fn apply(&self, chips: &mut Chips, mult: &mut Mult) -> () {
        let (c, m) = self.hand_value();
        // Add the values to the provided chips and multiplier
        *chips += c;
        *mult += m;
        explain!("{:?} {:?}", self, (chips, mult));
    }

    /// Recognize all poker hands that match the given cards
    fn recogonize(cards_played: &Vec<Card>) -> Vec<PokerHand> {
        // Get all possible poker hands in order
        let vs = Self::vector();
        let mut matches = Vec::new();
        // Check each hand type
        for v in vs {
            if v.is(&cards_played) {
                matches.push(v);
            }
        }
        matches
    }

    /// Determine if the given cards match this poker hand
    fn is(&self, cards_played: &Vec<Card>) -> bool {
        match *self {
            PokerHand::FlushFive =>
                // suite set might be 0 when 5 cards are all wild
                PokerHand::FiveOfAKind.is(cards_played) && same_suite(cards_played),

            PokerHand::FlushHouse =>
                PokerHand::FullHouse.is(cards_played) && same_suite(cards_played),

            PokerHand::FiveOfAKind => max_num_of_rank(cards_played) == 5,

            PokerHand::StraightFlush =>
                PokerHand::Flush.is(cards_played) && PokerHand::Straight.is(cards_played),

            PokerHand::FourOfAKind => max_num_of_rank(cards_played) >= 4,

            PokerHand::FullHouse =>
                PokerHand::ThreeOfAKind.is(cards_played) && PokerHand::TwoPair.is(cards_played),

            PokerHand::Flush => {
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

            PokerHand::Straight => {
                if cards_played.len() < 5 {
                    return false;
                }

                let mut cards = cards_played.clone();
                cards.sort_by_key(|card| card.rank.straight_value());

                // Check if cards are in sequential order
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

                // Special case for A-2-3-4-5 straight
                let has_ace = cards.iter().any(|card| matches!(card.rank, Rank::Ace));
                let has_two = cards.iter().any(|card| matches!(card.rank, Rank::Two));
                let has_three = cards.iter().any(|card| matches!(card.rank, Rank::Three));
                let has_four = cards.iter().any(|card| matches!(card.rank, Rank::Four));
                let has_five = cards.iter().any(|card| matches!(card.rank, Rank::Five));

                has_ace && has_two && has_three && has_four && has_five
            }

            PokerHand::ThreeOfAKind => max_num_of_rank(cards_played) >= 3,

            PokerHand::TwoPair => {
                let m = count_poker_by_rank(cards_played);
                let mut k = 0;
                for (_, count) in m {
                    if count >= 2 {
                        k += 1;
                    }
                    if k == 2 {
                        return true;
                    }
                }
                return false;
            }

            PokerHand::Pair => {
                let m = count_poker_by_rank(cards_played);
                for (_, count) in m {
                    // higher hands like five of a kind,
                    // four of a kind, three of a kind can also be considered as pair
                    if count >= 2 {
                        return true;
                    }
                }
                return false;
            }

            PokerHand::HighCard => max_num_of_rank(cards_played) == 1,
        }
    }

    /// Return vector of all poker hands in descending order of value
    fn vector() -> Vec<PokerHand> {
        return vec![
            PokerHand::FlushFive,
            PokerHand::FlushHouse,
            PokerHand::FiveOfAKind,
            PokerHand::StraightFlush,
            PokerHand::FourOfAKind,
            PokerHand::FullHouse,
            PokerHand::Flush,
            PokerHand::Straight,
            PokerHand::ThreeOfAKind,
            PokerHand::TwoPair,
            PokerHand::Pair,
            PokerHand::HighCard
        ];
    }
}
