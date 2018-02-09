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
    let flush = is_flush(hand.clone());
    let straight = is_straight(hand.clone());
    if straight && flush {
        HandRank::StraightFlush
    } else if flush {
        HandRank::Flush
    } else if straight {
        HandRank::Straight
    } else {
        HandRank::HighCard
    }
}

fn is_flush(hand: Vec<card::Card>) -> bool {
    let mut head = hand.clone();
    let rest = head.split_off(1);
    let first = head[0];
    rest.into_iter().all(|c| c.suit == first.suit)
}

fn is_straight(hand: Vec<card::Card>) -> bool {
    let mut is_straight = true;
    let mut sorted = hand.clone();
    sorted.sort();
    
    sorted.into_iter().enumerate().for_each(|(i, c)| {
        if is_straight && i < 4 {
            is_straight = c.rank.val() + 1 == hand[i + 1].rank.val();
        }
    });

    is_straight
}
