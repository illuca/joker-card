use crate::{
    explain,
    getters::StraghtValueGetter,
    utils::{ count_poker_by_rank, max_num_of_rank },
};

use ortalib::{ Card, Chips, Enhancement, Mult, PokerHand, Rank, Suit };
use std::{ collections::{ HashSet }, vec };

fn same_suite(cards_played: &Vec<Card>) -> bool {
    let mut suit_set: HashSet<Suit> = HashSet::new();
    for card in cards_played {
        match card.enhancement {
            Some(Enhancement::Wild) => (),
            _ => {
                suit_set.insert(card.suit);
            }
        }
    }
    suit_set.len() < 1
}

pub trait PokerHandUtils {
    fn is(&self, cards_played: &Vec<Card>) -> bool;
    fn recogonize(cards_played: &Vec<Card>) -> Vec<PokerHand>;
    fn vector() -> Vec<PokerHand>;
    // fn contains(&self, target: PokerHand, s: &Score) -> bool;
    fn apply(&self, chips: &mut Chips, mult: &mut Mult) -> ();
}
impl PokerHandUtils for PokerHand {
    fn apply(&self, chips: &mut Chips, mult: &mut Mult) -> () {
        let (c, m) = self.hand_value();
        *chips += c;
        *mult += m;
        explain!("{:?} {:?}", self, (chips, mult));
    }
    // fn contains(&self, target: PokerHand, s: &Score) -> bool {
    //     let vs = match self {

    //         PokerHand::FiveOfAKind => {
    //             let mut tmp = vec![
    //                 PokerHand::FiveOfAKind,
    //                 PokerHand::FourOfAKind,
    //                 PokerHand::ThreeOfAKind,
    //                 PokerHand::Pair
    //             ];
    //             PokerHand::Flush.is(&s.cards_played).then(|| tmp.push(PokerHand::Flush));
    //             tmp
    //         }
    //         PokerHand::FourOfAKind => {
    //             let mut tmp = vec![
    //                 PokerHand::FourOfAKind,
    //                 PokerHand::ThreeOfAKind,
    //                 PokerHand::Pair
    //             ];
    //             PokerHand::Flush.is(&s.cards_played).then(|| tmp.push(PokerHand::Flush));
    //             tmp
    //         }
    //         PokerHand::Flush => {
    //             let mut tmp: Vec<PokerHand> = Vec::new();
    //             PokerHand::Straight.is(&s.cards_played).then(|| tmp.push(PokerHand::Straight));
    //             PokerHand::ThreeOfAKind.is(&s.cards_played).then(|| tmp.push(PokerHand::Straight));
    //             PokerHand::TwoPair.is(&s.cards_played).then(|| tmp.push(PokerHand::TwoPair));
    //             tmp
    //         }
    //         PokerHand::ThreeOfAKind => vec![PokerHand::ThreeOfAKind, PokerHand::Pair],
    //         PokerHand::TwoPair => vec![PokerHand::TwoPair],
    //         PokerHand::Pair => vec![PokerHand::Pair],
    //         &x => vec![x],
    //     };

    //     return vs.contains(&target);
    // }
    fn recogonize(cards_played: &Vec<Card>) -> Vec<PokerHand> {
        let vs = Self::vector();
        let mut matches = Vec::new();
        for v in vs {
            if v.is(&cards_played) {
                matches.push(v);
            }
        }
        matches
    }

    fn is(&self, cards_played: &Vec<Card>) -> bool {
        // TODO check if it is 5 cards for StraightFlush, Flush,Straight
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
                    // five of a kind, flush
                    // four of a kind, three of a kind, two pair
                    // is possible to be a pair
                    if count >= 2 {
                        return true;
                    }
                }
                return false;
            }

            PokerHand::HighCard => max_num_of_rank(cards_played) == 1,
        }
    }

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
