#![allow(dead_code)]
use std::fmt;
use std::cmp::Ordering;

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

    pub fn val(self) -> u32 {
        match self {
            Rank::Two => 0,
            Rank::Three => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        }
    }
}

pub fn rank_from_str(rank: &str) -> Option<Rank> {
    match rank {
        "2" => Some(Rank::Two),
        "3" => Some(Rank::Three),
        "4" => Some(Rank::Four),
        "5" => Some(Rank::Five),
        "6" => Some(Rank::Six),
        "7" => Some(Rank::Seven),
        "8" => Some(Rank::Eight),
        "9" => Some(Rank::Nine),
        "T" => Some(Rank::Ten),
        "10" => Some(Rank::Ten),
        "J" => Some(Rank::Jack),
        "Q" => Some(Rank::Queen),
        "K" => Some(Rank::King),
        "A" => Some(Rank::Ace),
        //This is dumb
        _   => None,
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

pub fn suit_from_str(suit: &str) -> Option<Suit> {
    match suit {
        "S" => Some(Suit::Spades),
        "H" => Some(Suit::Hearts),
        "D" => Some(Suit::Diamonds),
        "C" => Some(Suit::Clubs),
        //This is dumb
        _  => None,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank.to_string(), self.suit.to_unicode_string())
    }
}

pub fn card_from_str(rank: &str, suit: &str) -> Card {
    Card {
        rank: rank_from_str(rank).expect("Invalid rank"),
        suit: suit_from_str(suit).expect("Invalid suit"),
    }
}
