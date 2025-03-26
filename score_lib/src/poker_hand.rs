use crate::{
    explain,
    getters::StraghtValueGetter,
    utils::{ count_poker_by_rank, max_num_of_rank },
};

use ortalib::{ Card, Chips, Enhancement, Mult, PokerHand, Rank, Round, Suit };
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
    fn is(&self, round: &Round) -> bool;
    fn recogonize(round: &Round) -> PokerHand;
    fn vector() -> Vec<PokerHand>;
    fn contains(&self, target: PokerHand) -> bool;
    fn apply(&self, chips: &mut Chips, mult: &mut Mult) -> ();
}
impl PokerHandUtils for PokerHand {
    fn apply(&self, chips: &mut Chips, mult: &mut Mult) -> () {
        let (c, m) = self.hand_value();
        *chips += c;
        *mult += m;
        explain!("{:?} {:?}", self, (chips, mult));
    }
    fn contains(&self, target: PokerHand) -> bool {
        let vs = match self {
            PokerHand::FiveOfAKind =>
                vec![
                    PokerHand::FiveOfAKind,
                    PokerHand::FourOfAKind,
                    PokerHand::ThreeOfAKind,
                    PokerHand::Pair
                ],
            PokerHand::FourOfAKind =>
                vec![
                    PokerHand::FourOfAKind,
                    PokerHand::ThreeOfAKind,
                    PokerHand::Pair,
                    PokerHand::Flush
                ],
            PokerHand::ThreeOfAKind => vec![PokerHand::ThreeOfAKind, PokerHand::Pair],
            PokerHand::TwoPair => vec![PokerHand::TwoPair],
            PokerHand::Pair => vec![PokerHand::Pair],
            &x => vec![x],
        };

        return vs.contains(&target);
    }
    fn recogonize(round: &Round) -> PokerHand {
        let vs = Self::vector();
        for v in vs {
            if v.is(&round) {
                return v;
            }
        }
        panic!("Not match any PokerHand.")
    }

    fn is(&self, round: &Round) -> bool {
        let cards_played = &round.cards_played;
        match *self {
            PokerHand::FlushFive => {
                // suite set might be 0 when 5 cards are all wild
                return PokerHand::FiveOfAKind.is(round) && same_suite(cards_played);
            }
            PokerHand::FlushHouse => {
                return PokerHand::FullHouse.is(round) && same_suite(cards_played);
            }
            PokerHand::FiveOfAKind => {
                return max_num_of_rank(cards_played) == 5;
            }
            PokerHand::StraightFlush => PokerHand::Flush.is(round) && PokerHand::Straight.is(round),
            PokerHand::FourOfAKind => {
                return max_num_of_rank(cards_played) == 4;
            }
            PokerHand::FullHouse => {
                return PokerHand::ThreeOfAKind.is(round) && PokerHand::TwoPair.is(round);
            }
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
            PokerHand::ThreeOfAKind => {
                return max_num_of_rank(cards_played) == 3;
            }
            PokerHand::TwoPair => {
                let m = count_poker_by_rank(cards_played);
                let mut k = 0;
                for (_, count) in m {
                    if count == 2 {
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
                    if count == 2 {
                        return true;
                    }
                }
                return false;
            }

            PokerHand::HighCard => {
                return max_num_of_rank(cards_played) == 1;
            }
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

pub trait HandPokerRecognizer {
    fn recognize_hand_type(&self) -> PokerHand;
}

impl HandPokerRecognizer for Round {
    fn recognize_hand_type(&self) -> PokerHand {
        for hand in PokerHand::vector() {
            if hand.is(&self) {
                return hand;
            }
        }
        PokerHand::HighCard
    }
}
