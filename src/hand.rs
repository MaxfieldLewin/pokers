use hand_rankings::HandRank;
use card::CardVec;

pub struct Hand {
    cards: Option(CardVec),
    rank: Option(HandRank),
    kickers: Option(Vec[u8: 5]),
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
