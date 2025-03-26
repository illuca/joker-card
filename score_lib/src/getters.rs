use ortalib::{ Card, Chips, Edition, Enhancement, Mult, PokerHand, Rank, Round };

use crate::utils::group_poker_by_rank;

pub trait EditionValueGetter {
    fn eidtion_value(&self, mult: Mult) -> (Chips, Mult, &str);
}

impl EditionValueGetter for Edition {
    fn eidtion_value(&self, mult: Mult) -> (Chips, Mult, &str) {
        match *self {
            Edition::Foil => (50.0, 0.0, "+50 Chips"),
            Edition::Holographic => (0.0, 10.0, "+10 Mult"),
            Edition::Polychrome => (0.0, 0.5 * mult, "x1.5 Mult"),
        }
    }
}

pub trait EnhancementValueGetter {
    fn enhancement_value(&self, mult: Mult, is_held_in_hand: bool) -> (Chips, Mult, &str);
}
impl EnhancementValueGetter for Enhancement {
    fn enhancement_value(&self, mult: Mult, is_held_in_hand: bool) -> (Chips, Mult, &str) {
        if !is_held_in_hand {
            match *self {
                Enhancement::Bonus => (30.0, 0.0, "+30 Chips"),
                Enhancement::Glass => (0.0, mult, "x2 Mult"),
                Enhancement::Mult => (0.0, 4.0, "+4 Mult"),
                _ => (0.0, 0.0, ""),
            }
        } else {
            match *self {
                Enhancement::Steel => (0.0, 0.5 * mult, "x1.5 Mult"),
                _ => (0.0, 0.0, ""),
            }
        }
    }
}

pub trait ScoredCardGetter {
    fn get_scored_cards(&self, poker_hand: PokerHand) -> Vec<Card>;
}
impl ScoredCardGetter for Round {
    fn get_scored_cards(&self, poker_hand: PokerHand) -> Vec<Card> {
        let mut cards = self.cards_played.clone();

        cards.sort_by(|a, b| b.rank.straight_value().cmp(&a.rank.straight_value()));

        match poker_hand {
            PokerHand::FourOfAKind => {
                let mut rank_groups = std::collections::HashMap::new();

                for card in &cards {
                    rank_groups.entry(card.rank).or_insert_with(Vec::new).push(*card);
                }

                for (_, group) in rank_groups.iter() {
                    if group.len() == 4 {
                        return group.clone();
                    }
                }

                vec![]
            }

            PokerHand::ThreeOfAKind => {
                let mut rank_groups = std::collections::HashMap::new();

                for card in &cards {
                    rank_groups.entry(card.rank).or_insert_with(Vec::new).push(*card);
                }

                // 查找三条
                let mut groups_by_size: Vec<_> = rank_groups.iter().collect();
                groups_by_size.sort_by(|a, b| {
                    let size_cmp = b.1.len().cmp(&a.1.len());
                    if size_cmp == std::cmp::Ordering::Equal {
                        b.0.straight_value().cmp(&a.0.straight_value())
                    } else {
                        size_cmp
                    }
                });

                for (_, group) in groups_by_size {
                    if group.len() == 3 {
                        return group.clone();
                    }
                }

                vec![]
            }
            PokerHand::TwoPair => {
                let rank_groups = group_poker_by_rank(&cards);

                let mut pairs: Vec<Vec<Card>> = rank_groups
                    .iter()
                    .filter(|(_, group)| group.len() >= 2)
                    .map(|(_, group)| {
                        let mut pair = group.clone();
                        pair.truncate(2); // 只取前两张牌
                        pair
                    })
                    .collect();

                pairs.sort_by(|a, b| b[0].rank.straight_value().cmp(&a[0].rank.straight_value()));

                if pairs.len() >= 2 {
                    let mut result = Vec::new();
                    result.extend_from_slice(&pairs[0]);
                    result.extend_from_slice(&pairs[1]);
                    return result;
                }

                vec![]
            }
            PokerHand::Pair => {
                let rank_groups = group_poker_by_rank(&cards);

                let mut pairs: Vec<_> = rank_groups
                    .iter()
                    .filter(|(_, group)| group.len() >= 2)
                    .collect();

                pairs.sort_by(|(a, _), (b, _)| b.straight_value().cmp(&a.straight_value()));

                if !pairs.is_empty() {
                    let mut pair = pairs[0].1.clone();
                    pair.truncate(2);
                    return pair;
                }

                vec![]
            }
            PokerHand::HighCard => {
                if
                    let Some(highest_card) = cards
                        .iter()
                        .max_by_key(|card| card.rank.straight_value())
                {
                    vec![*highest_card]
                } else {
                    vec![]
                }
            }
            _ => cards,
        }
    }
}

pub trait StraghtValueGetter {
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
