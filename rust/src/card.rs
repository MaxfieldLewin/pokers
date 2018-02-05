#![allow(dead_code)]
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

const RANKS: [Rank; 13] = [
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];

pub fn ranks() -> [Rank; 13] {
    RANKS
}

impl Rank {
    fn to_string(self) -> String {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }.to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

const SUITS: [Suit; 4] = [
    Suit::Spades,
    Suit::Hearts,
    Suit::Diamonds,
    Suit::Clubs,
];

pub fn suits() -> [Suit; 4] {
    SUITS
}

impl Suit {
    pub fn to_unicode_string(self) -> String {
        match self {
            Suit::Spades => "\u{2660}",
            Suit::Hearts => "\u{2665}",
            Suit::Diamonds => "\u{2666}",
            Suit::Clubs => "\u{2663}",
        }.to_string()
    }

    pub fn to_ascii_string(self) -> String {
        match self {
            Suit::Spades => "S",
            Suit::Hearts => "H",
            Suit::Diamonds => "D",
            Suit::Clubs => "C",
        }.to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank.to_string(), self.suit.to_unicode_string())
    }
}
