extern crate rand;

mod card;
mod deck;
mod hand_rankings;

#[cfg(test)]
mod tests {
    use card;
    use deck;
    use hand_rankings;
    use std::vec::Vec;
    
    fn high_card_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("2", "D"),
            card::card_from_str("3", "S"),
            card::card_from_str("4", "S"),
            card::card_from_str("5", "S"),
            card::card_from_str("7", "S"),
        ]
    }

    fn pair_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("2", "D"),
            card::card_from_str("2", "S"),
            card::card_from_str("3", "S"),
            card::card_from_str("4", "S"),
            card::card_from_str("5", "S"),
        ]
    }

    #[test]
    fn it_cards_from_str() {
        let c = card::card_from_str("K", "H");

        assert_eq!(card::Rank::King, c.rank);
        assert_eq!(card::Suit::Hearts, c.suit);
    }

    #[test]
    fn it_formats_cards() {
        let ace_of_spades = card::card_from_str("A", "S");
        
        assert_eq!("A of ♠", format!("{}", ace_of_spades));
    }

    #[test]
    fn it_creates_deck() {
        let d = deck::create_deck();
        let first_card = card::card_from_str("2", "S");
        let last_card = card::card_from_str("A", "C");

        assert_eq!(d.cards.len(), 52);
        assert_eq!(&first_card, d.cards.iter().next().unwrap());
        assert_eq!(&last_card, d.cards.iter().last().unwrap());
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
    fn it_knows_hand_rank_order() {
        let r = hand_rankings::hand_ranks();

        assert_eq!(r.iter().next().unwrap(), &hand_rankings::HandRank::HighCard);
        assert_eq!(r.iter().last().unwrap(), &hand_rankings::HandRank::StraightFlush);
    }

    #[test]
    fn it_detects_a_high_card() {
        let h = high_card_hand();

        assert_eq!(hand_rankings::rank_hand(h), hand_rankings::HandRank::HighCard);
    }

    fn it_detects_a_pair() {
        let h = pair_hand();

        assert_eq!(hand_rankings::rank_hand(h), hand_rankings::HandRank::Pair);
    }
}
