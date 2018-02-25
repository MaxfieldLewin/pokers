use hand_rankings::*;
use card::{CardVec, RankVec};

pub struct Hand {
    pub cards: CardVec,
    pub hand_rank: HandRank,
    pub kickers: RankVec,
}

impl Hand {

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

