use std::fmt;
use std::str::FromStr;
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

pub type RankVec = Vec<Rank>;

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
    fn to_string(self) -> &'static str {
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
        }
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

#[derive(Debug, Fail)]
#[fail(display = "Unknown card rank: {}", _0)]
pub struct RankParseError(String);

impl FromStr for Rank {
    type Err = RankParseError;

    fn from_str(rank: &str) -> Result<Self, RankParseError> {
        match rank.to_lowercase().as_ref() {
            "2" | "two" => Ok(Rank::Two),
            "3" | "three" => Ok(Rank::Three),
            "4" | "four" => Ok(Rank::Four),
            "5" | "five" => Ok(Rank::Five),
            "6" | "six" => Ok(Rank::Six),
            "7" | "seven" => Ok(Rank::Seven),
            "8" | "eight" => Ok(Rank::Eight),
            "9" | "nine" => Ok(Rank::Nine),
            "t" | "10" | "ten" => Ok(Rank::Ten),
            "j" | "jack" => Ok(Rank::Jack),
            "q" | "queen" => Ok(Rank::Queen),
            "k" | "king" => Ok(Rank::King),
            "a" | "ace" => Ok(Rank::Ace),
            _ => Err(RankParseError(rank.to_string())),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

const SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

pub fn suits() -> [Suit; 4] {
    SUITS
}

impl Suit {
    pub fn to_unicode_string(self) -> &'static str {
        match self {
            Suit::Spades => "\u{2660}",
            Suit::Hearts => "\u{2665}",
            Suit::Diamonds => "\u{2666}",
            Suit::Clubs => "\u{2663}",
        }
    }

    pub fn to_ascii_string(self) -> &'static str {
        match self {
            Suit::Spades => "S",
            Suit::Hearts => "H",
            Suit::Diamonds => "D",
            Suit::Clubs => "C",
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Unknown card suit: {}", _0)]
pub struct SuitParseError(String);

impl FromStr for Suit {
    type Err = SuitParseError;

    fn from_str(suit: &str) -> Result<Self, SuitParseError> {
        match suit.to_lowercase().as_ref() {
            "s" | "spades" | "\u{2660}" => Ok(Suit::Spades),
            "h" | "hearts" | "\u{2665}" => Ok(Suit::Hearts),
            "d" | "diamonds" | "\u{2666}" => Ok(Suit::Diamonds),
            "c" | "clubs" | "\u{2663}" => Ok(Suit::Clubs),
            _ => Err(SuitParseError(suit.to_string())),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub type CardVec = Vec<Card>;

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
        write!(
            f,
            "{} of {}",
            self.rank.to_string(),
            self.suit.to_unicode_string()
        )
    }
}
impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} of {}",
            self.rank.to_string(),
            self.suit.to_unicode_string()
        )
    }
}
pub fn card_from_str(rank: &str, suit: &str) -> Card {
    Card {
        rank: Rank::from_str(rank).unwrap(),
        suit: Suit::from_str(suit).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_cards_from_str() {
        let c = card_from_str("K", "H");

        assert_eq!(Rank::King, c.rank);
        assert_eq!(Suit::Hearts, c.suit);
    }

    #[test]
    fn it_formats_cards() {
        let ace_of_spades = card_from_str("A", "S");

        assert_eq!("A of ♠", format!("{}", ace_of_spades));
    }
}
