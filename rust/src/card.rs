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

pub fn rank_from_str(rank: &str) -> Rank {
    match rank {
        "2" => Rank::Two,
        "3" => Rank::Three,
        "4" => Rank::Four,
        "5" => Rank::Five,
        "6" => Rank::Six,
        "7" => Rank::Seven,
        "8" => Rank::Eight,
        "9" => Rank::Nine,
        "T" => Rank::Ten,
        "10" => Rank::Ten,
        "J" => Rank::Jack,
        "Q" => Rank::Queen,
        "K" => Rank::King,
        "A" => Rank::Ace,
        //This is dumb
        _   => Rank::Ace,
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

pub fn suit_from_str(suit: &str) -> Suit {
    match suit {
        "spades" => Suit::Spades,
        "s" => Suit::Spades,
        "S" => Suit::Spades,
        "hearts" => Suit::Hearts,
        "h" => Suit::Hearts,
        "H" => Suit::Hearts,
        "diamonds" => Suit::Diamonds,
        "d" => Suit::Diamonds,
        "D" => Suit::Diamonds,
        "clubs" => Suit::Clubs,
        "c" => Suit::Clubs,
        "C" => Suit::Clubs,
        //This is dumb
        _  => Suit::Clubs,
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

pub fn card_from_str(rank: &str, suit: &str) -> Card {
    Card {
        rank: rank_from_str(rank),
        suit: suit_from_str(suit),
    }
}
