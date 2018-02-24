extern crate rand;

pub mod card;
pub mod deck;
pub mod hand_rankings;

#[cfg(test)]
mod tests {
    use card::*;
    use deck::*;
    use hand_rankings::*;

    fn high_card_hand() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("3", "S"),
            card_from_str("4", "S"),
            card_from_str("5", "S"),
            card_from_str("7", "S"),
        ]
    }

    fn high_card_kickers() -> RankVec {
        vec![Rank::Seven, Rank::Five, Rank::Four, Rank::Three, Rank::Two]
    }

    fn pair_hand() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("4", "S"),
            card_from_str("2", "S"),
            card_from_str("3", "S"),
            card_from_str("5", "S"),
        ]
    }
    fn pair_kickers() -> RankVec {
        vec![Rank::Two, Rank::Five, Rank::Four, Rank::Three]
    }
    fn two_pair_hand() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("3", "S"),
            card_from_str("3", "D"),
            card_from_str("2", "S"),
            card_from_str("4", "S"),
        ]
    }
    fn two_pair_kickers() -> RankVec {
        vec![Rank::Three, Rank::Two, Rank::Four]
    }
    fn three_of_a_kind_hand() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("2", "S"),
            card_from_str("2", "H"),
            card_from_str("3", "D"),
            card_from_str("4", "S"),
        ]
    }
    fn three_of_a_kind_kickers() -> RankVec {
        vec![Rank::Two, Rank::Four, Rank::Three]
    }
    fn straight_hand() -> CardVec {
        vec![
            card_from_str("3", "D"),
            card_from_str("2", "S"),
            card_from_str("5", "S"),
            card_from_str("4", "H"),
            card_from_str("6", "D"),
        ]
    }
    fn straight_kickers() -> RankVec {
        vec![Rank::Six]
    }
    fn wheel_straight_hand() -> CardVec {
        vec![
            card_from_str("3", "D"),
            card_from_str("2", "S"),
            card_from_str("4", "H"),
            card_from_str("A", "D"),
            card_from_str("5", "S"),
        ]
    }
    fn wheel_straight_kickers() -> RankVec {
        vec![Rank::Five]
    }

    fn flush_hand() -> CardVec {
        vec![
            card_from_str("2", "S"),
            card_from_str("3", "S"),
            card_from_str("4", "S"),
            card_from_str("5", "S"),
            card_from_str("7", "S"),
        ]
    }
    fn flush_kickers() -> RankVec {
        vec![Rank::Seven, Rank::Five, Rank::Four, Rank::Three, Rank::Two]
    }
    fn full_house_hand() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("2", "S"),
            card_from_str("3", "D"),
            card_from_str("3", "S"),
            card_from_str("2", "H"),
        ]
    }
    fn full_house_kickers() -> RankVec {
        vec![Rank::Three, Rank::Two]
    }
    fn four_of_a_kind_hand() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("2", "H"),
            card_from_str("2", "C"),
            card_from_str("3", "D"),
            card_from_str("2", "S"),
        ]
    }
    fn four_of_a_kind_kickers() -> RankVec {
        vec![Rank::Two]
    }

    fn straight_flush_hand() -> CardVec {
        vec![
            card_from_str("2", "S"),
            card_from_str("3", "S"),
            card_from_str("5", "S"),
            card_from_str("6", "S"),
            card_from_str("4", "S"),
        ]
    }
    fn straight_flush_kickers() -> RankVec {
        vec![Rank::Six]
    }

    fn wheel_straight_flush_hand() -> CardVec {
        vec![
            card_from_str("2", "S"),
            card_from_str("3", "S"),
            card_from_str("5", "S"),
            card_from_str("A", "S"),
            card_from_str("4", "S"),
        ]
    }
    fn wheel_straight_flush_kickers() -> RankVec {
        vec![Rank::Five]
    }

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

    #[test]
    fn it_knows_hand_rank_order() {
        let r = hand_ranks();

        assert_eq!(*r.iter().next().unwrap(), HandRank::HighCard);
        assert_eq!(*r.iter().last().unwrap(), HandRank::StraightFlush);
    }

    #[test]
    fn it_detects_a_high_card() {
        let h = high_card_hand();

        assert_eq!(rank_hand(h), HandRank::HighCard);
    }

    #[test]
    fn it_gets_high_card_kickers() {
        let h = high_card_hand();

        assert_eq!(get_kickers(&h, HandRank::HighCard), high_card_kickers())
    }

    #[test]
    fn it_detects_a_pair() {
        let h = pair_hand();

        assert_eq!(rank_hand(h), HandRank::Pair);
    }

    #[test]
    fn it_gets_pair_kickers() {
        let h = pair_hand();

        assert_eq!(get_kickers(&h, HandRank::Pair), pair_kickers())
    }

    #[test]
    fn it_detects_a_two_pair() {
        let h = two_pair_hand();

        assert_eq!(rank_hand(h), HandRank::TwoPair);
    }

    #[test]
    fn it_gets_two_pair_kickers() {
        let h = two_pair_hand();

        assert_eq!(get_kickers(&h, HandRank::TwoPair), two_pair_kickers())
    }

    #[test]
    fn it_detects_a_three_of_a_kind() {
        let h = three_of_a_kind_hand();

        assert_eq!(rank_hand(h), HandRank::ThreeOfAKind);
    }

    #[test]
    fn it_gets_three_of_a_kind_kickers() {
        let h = three_of_a_kind_hand();

        assert_eq!(
            get_kickers(&h, HandRank::ThreeOfAKind),
            three_of_a_kind_kickers()
        )
    }

    #[test]
    fn it_detects_a_straight() {
        let h = straight_hand();

        assert_eq!(rank_hand(h), HandRank::Straight);
    }

    #[test]
    fn it_gets_straight_kickers() {
        let h = straight_hand();

        assert_eq!(get_kickers(&h, HandRank::Straight), straight_kickers())
    }

    #[test]
    fn it_detects_a_wheel() {
        let h = wheel_straight_hand();

        assert_eq!(rank_hand(h), HandRank::Straight);
    }

    #[test]
    fn it_gets_wheel_straight_kickers() {
        let h = wheel_straight_hand();

        assert_eq!(
            get_kickers(&h, HandRank::Straight),
            wheel_straight_kickers()
        )
    }

    #[test]
    fn it_detects_a_flush() {
        let h = flush_hand();

        assert_eq!(rank_hand(h), HandRank::Flush);
    }

    #[test]
    fn it_gets_flush_kickers() {
        let h = flush_hand();

        assert_eq!(get_kickers(&h, HandRank::Flush), flush_kickers())
    }

    #[test]
    fn it_detects_a_full_house() {
        let h = full_house_hand();

        assert_eq!(rank_hand(h), HandRank::FullHouse);
    }

    #[test]
    fn it_gets_full_house_kickers() {
        let h = full_house_hand();

        assert_eq!(get_kickers(&h, HandRank::FullHouse), full_house_kickers())
    }

    #[test]
    fn it_detects_a_four_of_a_kind() {
        let h = four_of_a_kind_hand();

        assert_eq!(rank_hand(h), HandRank::FourOfAKind);
    }

    #[test]
    fn it_gets_four_of_a_kind_kickers() {
        let h = four_of_a_kind_hand();

        assert_eq!(
            get_kickers(&h, HandRank::FourOfAKind),
            four_of_a_kind_kickers()
        )
    }

    #[test]
    fn it_detects_a_straight_flush() {
        let h = straight_flush_hand();

        assert_eq!(rank_hand(h), HandRank::StraightFlush);
    }

    #[test]
    fn it_gets_straight_flush_kickers() {
        let h = straight_flush_hand();

        assert_eq!(
            get_kickers(&h, HandRank::StraightFlush),
            straight_flush_kickers()
        )
    }
    #[test]
    fn it_detects_a_wheel_straight_flush() {
        let h = wheel_straight_flush_hand();

        assert_eq!(rank_hand(h), HandRank::StraightFlush);
    }

    #[test]
    fn it_gets_wheel_straight_flush_kickers() {
        let h = wheel_straight_flush_hand();

        assert_eq!(
            get_kickers(&h, HandRank::StraightFlush),
            wheel_straight_flush_kickers()
        )
    }
}
