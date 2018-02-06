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

// Assuming 5 cards
pub fn rank_hand(hand: Vec<card::Card>) -> HandRank {
    //HandRank::HighCard
    // Flush
    // Straight
    // (StraightFlush)
    //
    // Quads
    // Trips 
    // Pairs
    // (HC)
}

fn is_flush(hand: Vec<card::Card>) -> bool {

}

fn is_straight(hand: Vec<card::Card>) -> bool {
}
