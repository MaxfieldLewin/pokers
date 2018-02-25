use hand_rankings::HandRank;
use card::{CardVec, RankVec};

pub struct Hand {
    cards: Option(CardVec),
    rank: Option(HandRank),
    kickers: Option(RankVec),
}

impl Hand {

}

fn init_empty_hand() -> Hand {
    Hand {
        cards: None,
        rank: None,
        kickers: None,
    }
}
