extern crate rand;

mod card;
mod deck;
//mod hand;
//mod hand_rankings;

#[cfg(test)]
mod tests {
    use card;
    use deck;
    #[test]
    fn it_formats_cards() {
        let ace_of_spades = card::Card {
            suit: card::Suit::Spades,
            rank: card::Rank::Ace,
        };
        
        assert_eq!("A of ♠", format!("{}", ace_of_spades));
    }

    #[test]
    fn it_creates_deck() {
        let d = deck::create_deck();
        let first_card = card::Card {
            suit: card::Suit::Spades,
            rank: card::Rank::Two,
        };

        assert_eq!(d.cards.len(), 52);
        assert_eq!(&first_card, d.cards.iter().next().unwrap());
    }

    #[test]
    fn it_creates_shuffled_deck() {
        let d = deck::create_shuffled_deck();
        println!("{}", d);
        assert_eq!(d.cards.len(), 52);
    }
}
