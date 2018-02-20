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

    fn two_pair_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("2", "D"),
            card::card_from_str("2", "S"),
            card::card_from_str("3", "S"),
            card::card_from_str("3", "D"),
            card::card_from_str("4", "S"),
        ]
    }

    fn three_of_a_kind_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("2", "D"),
            card::card_from_str("2", "S"),
            card::card_from_str("2", "H"),
            card::card_from_str("3", "D"),
            card::card_from_str("4", "S"),
        ]
    }

    fn straight_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("3", "D"),
            card::card_from_str("2", "S"),
            card::card_from_str("5", "S"),
            card::card_from_str("4", "H"),
            card::card_from_str("6", "D"),
        ]
    }

    fn wheel_straight_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("3", "D"),
            card::card_from_str("2", "S"),
            card::card_from_str("4", "H"),
            card::card_from_str("A", "D"),
            card::card_from_str("5", "S"),
        ]
    }

    fn flush_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("2", "S"),
            card::card_from_str("3", "S"),
            card::card_from_str("4", "S"),
            card::card_from_str("5", "S"),
            card::card_from_str("7", "S"),
        ]
    }

    fn full_house_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("2", "D"),
            card::card_from_str("2", "S"),
            card::card_from_str("2", "H"),
            card::card_from_str("3", "D"),
            card::card_from_str("3", "S"),
        ]
    }

    fn four_of_a_kind_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("2", "D"),
            card::card_from_str("2", "S"),
            card::card_from_str("2", "H"),
            card::card_from_str("2", "C"),
            card::card_from_str("3", "D"),
        ]
    }

    fn straight_flush_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("2", "S"),
            card::card_from_str("3", "S"),
            card::card_from_str("5", "S"),
            card::card_from_str("6", "S"),
            card::card_from_str("4", "S"),
        ]
    }

    fn wheel_straight_flush_hand() -> Vec<card::Card> {
        vec![
            card::card_from_str("2", "S"),
            card::card_from_str("3", "S"),
            card::card_from_str("5", "S"),
            card::card_from_str("A", "S"),
            card::card_from_str("4", "S"),
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
        assert_eq!(
            r.iter().last().unwrap(),
            &hand_rankings::HandRank::StraightFlush
        );
    }

    #[test]
    fn it_detects_a_high_card() {
        let h = high_card_hand();

        assert_eq!(
            hand_rankings::rank_hand(h),
            hand_rankings::HandRank::HighCard
        );
    }

    #[test]
    fn it_detects_a_pair() {
        let h = pair_hand();

        assert_eq!(hand_rankings::rank_hand(h), hand_rankings::HandRank::Pair);
    }

    #[test]
    fn it_detects_a_two_pair() {
        let h = two_pair_hand();

        assert_eq!(
            hand_rankings::rank_hand(h),
            hand_rankings::HandRank::TwoPair
        );
    }

    #[test]
    fn it_detects_a_three_of_a_kind() {
        let h = three_of_a_kind_hand();

        assert_eq!(
            hand_rankings::rank_hand(h),
            hand_rankings::HandRank::ThreeOfAKind
        );
    }

    #[test]
    fn it_detects_a_straight() {
        let h = straight_hand();

        assert_eq!(
            hand_rankings::rank_hand(h),
            hand_rankings::HandRank::Straight
        );
    }

    #[test]
    fn it_detects_a_wheel() {
        let h = wheel_straight_hand();

        assert_eq!(
            hand_rankings::rank_hand(h),
            hand_rankings::HandRank::Straight
        );
    }

    #[test]
    fn it_detects_a_flush() {
        let h = flush_hand();

        assert_eq!(hand_rankings::rank_hand(h), hand_rankings::HandRank::Flush);
    }

    #[test]
    fn it_detects_a_four_of_a_kind() {
        let h = four_of_a_kind_hand();

        assert_eq!(
            hand_rankings::rank_hand(h),
            hand_rankings::HandRank::FourOfAKind
        );
    }

    #[test]
    fn it_detects_a_straight_flush() {
        let h = straight_flush_hand();

        assert_eq!(
            hand_rankings::rank_hand(h),
            hand_rankings::HandRank::StraightFlush
        );
    }

    #[test]
    fn it_detects_a_wheel_straight_flush() {
        let h = wheel_straight_flush_hand();

        assert_eq!(
            hand_rankings::rank_hand(h),
            hand_rankings::HandRank::StraightFlush
        );
    }
}
