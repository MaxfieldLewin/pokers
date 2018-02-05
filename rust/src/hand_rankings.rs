#![allow(dead_code)]

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

const HAND_RANKS : [HandRank; 9] = [
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
