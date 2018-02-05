extern crate rand;

mod card;
mod deck;
mod hand_rankings;

#[cfg(test)]
mod tests {
    use card;
    use deck;
    use hand_rankings;
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
        //println!("Does this deck look shuffled? \n{}", d);
        assert_eq!(d.cards.len(), 52);
    }

    #[test]
    fn it_deals_cards() {
        let mut d = deck::create_shuffled_deck();
        let h = d.deal_cards(5);
        
        assert_eq!(d.cards.len(), 47);
        assert_eq!(h.len(), 5);
    }

    #[test]
    fn it_knows_hand_ranks() {
        let r = hand_rankings::hand_ranks();

        assert_eq!(r.iter().next().unwrap(), &hand_rankings::HandRank::HighCard);
        assert_eq!(r.iter().last().unwrap(), &hand_rankings::HandRank::StraightFlush);
    }
}
