use std::cmp::Ordering;
use hand_rankings::*;
use card::{CardVec, RankVec};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Hand {
    pub cards: CardVec,
    pub hand_rank: HandRank,
    pub kickers: RankVec,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let hand_rank_cmp = self.hand_rank.partial_cmp(&other.hand_rank);
        if hand_rank_cmp == Some(Ordering::Equal) {
            self.kickers.partial_cmp(&other.kickers)
        } else {
            hand_rank_cmp
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.hand_rank.eq(&other.hand_rank) && self.kickers.eq(&other.kickers)
    }
}

pub fn init_hand(cards: CardVec) -> Hand {
    let hand_rank = rank_hand(&cards);
    let kickers = get_kickers(&cards, hand_rank);

    Hand {
        cards,
        hand_rank,
        kickers,
    }
}

pub fn find_best_hand(cards: CardVec) -> Option<Hand> {
    if cards.len() < 5 {
        None
    } else {
        let mut hand_vec: Vec<Hand> = cards
            .iter()
            .cloned()
            .combinations(5)
            .map(|cv| init_hand(cv))
            .collect();
        hand_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        hand_vec.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;
    use card::card_from_str;

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

    #[test]
    fn it_compares_straight_hands() {
        let six_high_straight = init_hand(straight_hand());
        let seven_high_straight = init_hand(straight_hand_2());

        assert_ne!(seven_high_straight, six_high_straight);
        assert!(seven_high_straight > six_high_straight);
    }

    #[test]
    fn it_tiebreaks_straight_hands() {
        let seven_high_straight = init_hand(straight_hand_2());
        let seven_high_straight_2 = init_hand(straight_hand_3());

        assert_eq!(seven_high_straight, seven_high_straight_2);
    }

    // TODO: full house, quads, straight flush
    #[test]
    fn it_compares_flush_hands() {
        let seven_high_flush = init_hand(flush_hand());
        let eight_high_flush = init_hand(flush_hand_2());

        assert_ne!(eight_high_flush, seven_high_flush);
        assert!(eight_high_flush > seven_high_flush);
    }

    #[test]
    fn it_tiebreaks_flush_hands() {
        let eight_high_flush = init_hand(flush_hand_2());
        let eight_high_flush_2 = init_hand(flush_hand_3());

        assert_ne!(eight_high_flush_2, eight_high_flush);
        assert!(eight_high_flush_2 > eight_high_flush);
    }

    #[test]
    fn it_find_the_best_hand() {
        let board = vec![
            card_from_str("A", "S"),
            card_from_str("A", "C"),
            card_from_str("A", "D"),
            card_from_str("J", "D"),
            card_from_str("Q", "S"),
            card_from_str("K", "D"),
            card_from_str("T", "C"),
        ];

        let best_hand = find_best_hand(board).unwrap();
        assert_eq!(best_hand.hand_rank, HandRank::Straight);
    }
}
