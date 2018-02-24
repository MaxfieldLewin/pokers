use std::vec::Vec;
use std::fmt;

use rand;
use card;

type CardVec = Vec<card::Card>;

pub struct Deck {
    pub cards: CardVec,
}

impl Deck {
    pub fn deal_cards(&mut self, count: u32) -> CardVec {
        let mut cards = vec![];
        for _ in 0..count {
            cards.push(self.cards.pop().expect("Ran out of cards in the deck!"));
        }

        cards 
    }

    pub fn shuffle(&mut self) {
        use rand::Rng;
        rand::thread_rng().shuffle(&mut self.cards);
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_string = "".to_string();
        for card in &self.cards {
            display_string.push_str(&card.to_string());
            display_string.push_str("\n");
        }

        write!(f, "{}", display_string)
    }
}

pub fn create_deck() -> Deck {
    let mut cards = vec![];

    for suit in &card::suits() {
        for rank in &card::ranks() {
            cards.push(card::Card {
                suit: *suit,
                rank: *rank,
            });
        }
    }

    Deck { cards }
}

pub fn create_shuffled_deck() -> Deck {
    let mut d = create_deck();
    d.shuffle();
    d
}
