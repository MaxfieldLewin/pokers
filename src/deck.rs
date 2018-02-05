use std::vec;
use std::fmt;

use rand;

use card;

pub struct Deck {
    pub cards: vec::Vec<card::Card>, 
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
    use rand::Rng;
    let mut deck = create_deck();

    rand::thread_rng().shuffle(&mut deck.cards);
    deck
}

