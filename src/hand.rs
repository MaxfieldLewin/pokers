use std::cmp::Ordering;

use hand_rankings::*;
use card::{CardVec, RankVec};

#[derive(Clone, Debug)]
pub struct Hand {
    pub cards: CardVec,
    pub hand_rank: HandRank,
    pub kickers: RankVec,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let hand_rank_cmp = self.hand_rank.partial_cmp(&other.hand_rank);
        if hand_rank_cmp == Some(Ordering::Equal) {
            self.kickers.partial_cmp(&other.kickers)
        } else {
            hand_rank_cmp
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.hand_rank.eq(&other.hand_rank) && self.kickers.eq(&other.kickers)
    }
}

pub fn init_hand(cards: CardVec) -> Hand {
    let hand_rank = rank_hand(&cards);
    let kickers = get_kickers(&cards, hand_rank);

    Hand {
        cards,
        hand_rank,
        kickers,
    }
}
