extern crate rand;
extern crate failure;
#[macro_use] extern crate failure_derive;


pub mod card;
pub mod deck;
pub mod hand_rankings;
pub mod hand;
pub mod game;

#[cfg(test)]
mod tests {
    use card::*;
    use deck::*;
    use hand_rankings::*;
    use hand::*;

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
    // 8 high, five kicker
    fn high_card_hand_2() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("3", "S"),
            card_from_str("4", "S"),
            card_from_str("5", "S"),
            card_from_str("8", "S"),
        ]
    }
    // 8 high, six kicker
    fn high_card_hand_3() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("3", "S"),
            card_from_str("4", "S"),
            card_from_str("6", "S"),
            card_from_str("8", "S"),
        ]
    }
    // 8 high, six kicker, different suits
    fn high_card_hand_4() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("3", "H"),
            card_from_str("4", "S"),
            card_from_str("6", "S"),
            card_from_str("8", "S"),
        ]
    }
    // Pair is lower than kickers
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

    // Pair is between kickers
    fn pair_hand_2() -> CardVec {
        vec![
            card_from_str("5", "D"),
            card_from_str("6", "S"),
            card_from_str("2", "S"),
            card_from_str("3", "S"),
            card_from_str("5", "S"),
        ]
    }

    fn pair_2_kickers() -> RankVec {
        vec![Rank::Five, Rank::Six, Rank::Three, Rank::Two]
    }

    // Pair is higher than kickers
    fn pair_hand_3() -> CardVec {
        vec![
            card_from_str("5", "D"),
            card_from_str("4", "S"),
            card_from_str("2", "S"),
            card_from_str("3", "S"),
            card_from_str("5", "S"),
        ]
    }

    fn pair_3_kickers() -> RankVec {
        vec![Rank::Five, Rank::Four, Rank::Three, Rank::Two]
    }
    // the lowliest possible two pair hand
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

    // aces and threes with a four kicker
    fn two_pair_hand_2() -> CardVec {
        vec![
            card_from_str("3", "S"),
            card_from_str("3", "D"),
            card_from_str("A", "D"),
            card_from_str("A", "C"),
            card_from_str("4", "S"),
        ]
    }
    // aces and threes with a king kicker
    fn two_pair_hand_3() -> CardVec {
        vec![
            card_from_str("3", "S"),
            card_from_str("3", "D"),
            card_from_str("A", "D"),
            card_from_str("A", "C"),
            card_from_str("K", "S"),
        ]
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
    fn three_of_a_kind_hand_2() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("3", "S"),
            card_from_str("4", "H"),
            card_from_str("4", "D"),
            card_from_str("4", "S"),
        ]
    }
    fn three_of_a_kind_hand_3() -> CardVec {
        vec![
            card_from_str("2", "D"),
            card_from_str("A", "S"),
            card_from_str("4", "H"),
            card_from_str("4", "D"),
            card_from_str("4", "S"),
        ]
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
        vec![Rank::Two, Rank::Three]
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
        vec![Rank::Two, Rank::Three]
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

        assert_eq!(rank_hand(&h), HandRank::HighCard);
    }

    #[test]
    fn it_gets_high_card_kickers() {
        let h = high_card_hand();

        assert_eq!(get_kickers(&h, HandRank::HighCard), high_card_kickers())
    }

    #[test]
    fn it_detects_a_underpair() {
        let h = pair_hand();

        assert_eq!(rank_hand(&h), HandRank::Pair);
    }

    #[test]
    fn it_gets_underpair_kickers() {
        let h = pair_hand();

        assert_eq!(get_kickers(&h, HandRank::Pair), pair_kickers())
    }
    #[test]
    fn it_detects_a_midpair() {
        let h = pair_hand_2();

        assert_eq!(rank_hand(&h), HandRank::Pair);
    }
    #[test]
    fn it_gets_midpair_kickers() {
        let h = pair_hand_2();

        assert_eq!(get_kickers(&h, HandRank::Pair), pair_2_kickers())
    }
    #[test]
    fn it_detects_a_highpair() {
        let h = pair_hand_3();

        assert_eq!(rank_hand(&h), HandRank::Pair);
    }
    #[test]
    fn it_gets_highpair_kickers() {
        let h = pair_hand_3();

        assert_eq!(get_kickers(&h, HandRank::Pair), pair_3_kickers())
    }
    #[test]
    fn it_detects_a_two_pair() {
        let h = two_pair_hand();

        assert_eq!(rank_hand(&h), HandRank::TwoPair);
    }

    #[test]
    fn it_gets_two_pair_kickers() {
        let h = two_pair_hand();

        assert_eq!(get_kickers(&h, HandRank::TwoPair), two_pair_kickers())
    }

    #[test]
    fn it_detects_a_three_of_a_kind() {
        let h = three_of_a_kind_hand();

        assert_eq!(rank_hand(&h), HandRank::ThreeOfAKind);
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

        assert_eq!(rank_hand(&h), HandRank::Straight);
    }

    #[test]
    fn it_gets_straight_kickers() {
        let h = straight_hand();

        assert_eq!(get_kickers(&h, HandRank::Straight), straight_kickers())
    }

    #[test]
    fn it_detects_a_wheel() {
        let h = wheel_straight_hand();

        assert_eq!(rank_hand(&h), HandRank::Straight);
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

        assert_eq!(rank_hand(&h), HandRank::Flush);
    }

    #[test]
    fn it_gets_flush_kickers() {
        let h = flush_hand();

        assert_eq!(get_kickers(&h, HandRank::Flush), flush_kickers())
    }

    #[test]
    fn it_detects_a_full_house() {
        let h = full_house_hand();

        assert_eq!(rank_hand(&h), HandRank::FullHouse);
    }

    #[test]
    fn it_gets_full_house_kickers() {
        let h = full_house_hand();

        assert_eq!(get_kickers(&h, HandRank::FullHouse), full_house_kickers())
    }

    #[test]
    fn it_detects_a_four_of_a_kind() {
        let h = four_of_a_kind_hand();

        assert_eq!(rank_hand(&h), HandRank::FourOfAKind);
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

        assert_eq!(rank_hand(&h), HandRank::StraightFlush);
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

        assert_eq!(rank_hand(&h), HandRank::StraightFlush);
    }

    #[test]
    fn it_gets_wheel_straight_flush_kickers() {
        let h = wheel_straight_flush_hand();

        assert_eq!(
            get_kickers(&h, HandRank::StraightFlush),
            wheel_straight_flush_kickers()
        )
    }

    // HAND
    #[test]
    fn it_inits_a_hand() {
        let cards = high_card_hand();
        let hand = init_hand(cards.clone());

        assert_eq!(hand.cards, cards);
        assert_eq!(hand.hand_rank, HandRank::HighCard);
        assert_eq!(hand.kickers, high_card_kickers());
    }

    #[test]
    fn it_finds_identical_hand_equal() {
        let h1 = init_hand(high_card_hand().clone());
        let h2 = init_hand(high_card_hand().clone());

        assert_eq!(h1, h2);
    }
    #[test]
    fn it_ignores_suit_difference_in_equal_hands() {
        let h1 = init_hand(high_card_hand_3().clone());
        let h2 = init_hand(high_card_hand_4().clone());

        assert_eq!(h1, h2);
    }
    #[test]
    fn it_finds_hands_of_differing_rank_unequal() {
        let h1 = init_hand(high_card_hand().clone());
        let h2 = init_hand(pair_hand().clone());

        assert_ne!(h1, h2);
    }
    #[test]
    fn it_compares_high_card_hands() {
        let seven_high = init_hand(high_card_hand().clone());
        let eight_high = init_hand(high_card_hand_2().clone());

        assert_ne!(seven_high, eight_high);
        assert!(eight_high > seven_high);
    }
    #[test]
    fn it_tiebreaks_high_card_hands() {
        let eight_high_six_kicker = init_hand(high_card_hand_3().clone());
        let eight_high_five_kicker = init_hand(high_card_hand_2().clone());

        assert_ne!(eight_high_six_kicker, eight_high_five_kicker);
        assert!(eight_high_six_kicker > eight_high_five_kicker);
    }
    #[test]
    fn it_compares_pair_hands() {
        let pair_twos = init_hand(pair_hand().clone());
        let pair_fives = init_hand(pair_hand_2().clone());

        assert_ne!(pair_fives, pair_twos);
        assert!(pair_fives > pair_twos);
    }
    #[test]
    fn it_tiebreaks_pair_hands() {
        let pair_fives_seven_kicker = init_hand(pair_hand_2().clone());
        let pair_fives_four_kicker = init_hand(pair_hand_3().clone());

        assert_ne!(pair_fives_seven_kicker, pair_fives_four_kicker);
        assert!(pair_fives_seven_kicker > pair_fives_four_kicker);
    }
    #[test]
    fn it_compares_two_pair_hands() {
        let threes_and_twos = init_hand(two_pair_hand().clone());
        let aces_and_threes = init_hand(two_pair_hand_2().clone());

        assert_ne!(aces_and_threes, threes_and_twos);
        assert!(aces_and_threes > threes_and_twos);
    }
    #[test]
    fn it_tiebreaks_two_pair_hands() {
        let aces_and_threes_four = init_hand(two_pair_hand_2().clone());
        let aces_and_threes_king = init_hand(two_pair_hand_3().clone());

        assert_ne!(aces_and_threes_king, aces_and_threes_four);
        assert!(aces_and_threes_king > aces_and_threes_four);
    }
    #[test]
    fn it_compares_three_of_a_kind_hands() {
        let set_twos = init_hand(three_of_a_kind_hand().clone());
        let set_fours = init_hand(three_of_a_kind_hand_2().clone());

        assert_ne!(set_fours, set_twos);
        assert!(set_fours > set_twos);
    }
    #[test]
    fn it_tiebreaks_three_of_a_kind_hands() {
        let set_fours_three_kicker = init_hand(three_of_a_kind_hand_2().clone());
        let set_fours_ace_kicker = init_hand(three_of_a_kind_hand_3().clone());
        assert_ne!(set_fours_ace_kicker, set_fours_three_kicker);
        assert!(set_fours_ace_kicker > set_fours_three_kicker);
    }

    // TODO: straight, flush, full house, quads,
}
