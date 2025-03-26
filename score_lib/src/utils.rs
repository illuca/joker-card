use std::collections::HashMap;

use ortalib::{ Card, Rank };

pub fn group_poker_by_rank(cards_played: &Vec<Card>) -> HashMap<Rank, Vec<Card>> {
    let mut rank_groups = HashMap::new();
    for card in cards_played {
        rank_groups.entry(card.rank).or_insert_with(Vec::new).push(*card);
    }
    return rank_groups;
}

pub fn count_poker_by_rank(cards_played: &Vec<Card>) -> HashMap<Rank, i32> {
    let mut m: HashMap<Rank, i32> = HashMap::new();
    for card in cards_played {
        *m.entry(card.rank).or_insert(0) += 1;
    }
    return m;
}

pub fn max_num_of_rank(cards_played: &Vec<Card>) -> i32 {
    let m = count_poker_by_rank(cards_played);
    if let Some((&_, &count)) = m.iter().max_by_key(|&(_, count)| count) {
        return count;
    } else {
        return 0;
    }
}
