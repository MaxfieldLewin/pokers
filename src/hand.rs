use hand_rankings::HandRank;
use card::CardVec;

pub struct Hand {
    cards: CardVec,
    rank: HandRank,
    kickers: Vec[u8: 5],
}
