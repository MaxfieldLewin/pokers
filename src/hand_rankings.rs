use card::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

const HAND_RANKS: [HandRank; 9] = [
    HandRank::HighCard,
    HandRank::Pair,
    HandRank::TwoPair,
    HandRank::ThreeOfAKind,
    HandRank::Straight,
    HandRank::Flush,
    HandRank::FullHouse,
    HandRank::FourOfAKind,
    HandRank::StraightFlush,
];

pub fn hand_ranks() -> [HandRank; 9] {
    HAND_RANKS
}

// Assuming 5 cards currently
pub fn rank_hand(hand: CardVec) -> HandRank {
    let mut hand = hand.clone();
    hand.sort();

    let flush = is_flush(&hand);
    let straight = is_straight(&hand);
    let buckets = get_buckets(&hand);

    if straight && flush {
        HandRank::StraightFlush
    } else if buckets == FOUROFAKIND_BUCKETS {
        HandRank::FourOfAKind
    } else if buckets == FULLHOUSE_BUCKETS {
        HandRank::FullHouse
    } else if flush {
        HandRank::Flush
    } else if straight {
        HandRank::Straight
    } else if buckets == THREEOFAKIND_BUCKETS {
        HandRank::ThreeOfAKind
    } else if buckets == TWOPAIR_BUCKETS {
        HandRank::TwoPair
    } else if buckets == PAIR_BUCKETS {
        HandRank::Pair
    } else {
        HandRank::HighCard
    }
}

fn is_flush(hand: &CardVec) -> bool {
    let first = hand[0];
    hand.iter().all(|c| c.suit == first.suit)
}

fn is_straight(hand: &CardVec) -> bool {
    let mut is_straight = true;

    hand.iter().enumerate().for_each(|(i, c)| {
        //Wheel case
        if i == 3 && is_straight && c.rank == Rank::Five
            && hand[i + 1].rank == Rank::Ace
        {
            ;
        } else if is_straight && i < 4 {
            is_straight = c.rank.val() + 1 == hand[i + 1].rank.val();
        }
    });

    is_straight
}

const PAIR_BUCKETS: [u8; 4] = [3, 1, 0, 0];
const TWOPAIR_BUCKETS: [u8; 4] = [1, 2, 0, 0];
const THREEOFAKIND_BUCKETS: [u8; 4] = [2, 0, 1, 0];
const FULLHOUSE_BUCKETS: [u8; 4] = [0, 1, 1, 0];
const FOUROFAKIND_BUCKETS: [u8; 4] = [1, 0, 0, 1];

fn get_buckets(hand: &CardVec) -> [u8; 4] {
    let mut buckets = [0, 0, 0, 0];
    let mut acc = 1;

    hand.iter().enumerate().for_each(|(i, c)| {
        if i < 4 && c.rank == hand[i + 1].rank {
            acc += 1;
        } else {
            buckets[acc - 1] += 1;
            acc = 1;
        }
    });

    buckets
}
