use rand::{thread_rng, Rng};
use std::fmt;

use card::{ranks, suits, Card, CardVec};

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
        thread_rng().shuffle(&mut self.cards);
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

    for suit in &suits() {
        for rank in &ranks() {
            cards.push(Card {
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

#[cfg(test)]
mod tests {
    use super::*;
    use card::card_from_str;

    #[test]
    fn it_creates_deck() {
        let d = create_deck();
        let first_card = card_from_str("2", "S");
        let last_card = card_from_str("A", "C");

        assert_eq!(d.cards.len(), 52);
        assert_eq!(&first_card, d.cards.iter().next().unwrap());
        assert_eq!(&last_card, d.cards.iter().last().unwrap());
    }

    #[test]
    fn it_creates_shuffled_deck() {
        let d = create_shuffled_deck();
        //println!("Does this deck look shuffled? \n{}", d);
        assert_eq!(d.cards.len(), 52);
    }

    #[test]
    fn it_deals_cards() {
        let mut d = create_shuffled_deck();
        let h = d.deal_cards(5);

        assert_eq!(d.cards.len(), 47);
        assert_eq!(h.len(), 5);
    }

}
