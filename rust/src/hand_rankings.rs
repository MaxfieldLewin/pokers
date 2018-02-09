#![allow(dead_code)]
use std::vec::Vec;
use card;

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

// Assuming 5 cards currently
pub fn rank_hand(hand: Vec<card::Card>) -> HandRank {
    // Flush
    // Straight
    // (StraightFlush)
    // Pair
    // TwoPair
    // ThreeOfAkind
    // FullHouse
    // FourOfAKind
    // (HighCard)
    if is_flush(hand) {
        HandRank::Flush
    } else {
        HandRank::HighCard
    }
}

fn is_flush(hand: Vec<card::Card>) -> bool {
    let mut first = hand.clone();
    let mut rest  = first.split_off(1);
    rest.into_iter().all(|c| c.suit == first[0].suit)
}

fn is_straight(hand: Vec<card::Card>) -> bool {
    false
}
