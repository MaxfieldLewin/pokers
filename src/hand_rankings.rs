use std::collections::VecDeque;
use card::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

const PAIR_BUCKETS: [u8; 4] = [3, 1, 0, 0];
const TWOPAIR_BUCKETS: [u8; 4] = [1, 2, 0, 0];
const THREEOFAKIND_BUCKETS: [u8; 4] = [2, 0, 1, 0];
const FULLHOUSE_BUCKETS: [u8; 4] = [0, 1, 1, 0];
const FOUROFAKIND_BUCKETS: [u8; 4] = [1, 0, 0, 1];
const UNPAIRED_BUCKETS: [u8; 4] = [5, 0, 0, 0];

// Assuming 5 cards currently
pub fn rank_hand(hand: &CardVec) -> HandRank {
    let mut hand = hand.clone();
    hand.sort();

    let flush = is_flush(&hand);
    let straight = is_straight(&hand);
    let buckets = get_buckets(&hand);

    match buckets {
        UNPAIRED_BUCKETS if flush && straight => HandRank::StraightFlush,
        FOUROFAKIND_BUCKETS => HandRank::FourOfAKind,
        FULLHOUSE_BUCKETS => HandRank::FullHouse,
        UNPAIRED_BUCKETS if flush => HandRank::Flush,
        UNPAIRED_BUCKETS if straight => HandRank::Straight,
        THREEOFAKIND_BUCKETS => HandRank::ThreeOfAKind,
        TWOPAIR_BUCKETS => HandRank::TwoPair,
        PAIR_BUCKETS => HandRank::Pair,
        UNPAIRED_BUCKETS => HandRank::HighCard,
        // TODO: Make buckets enum so this could be exahustive? Idk if it can solve the bools
        _ => HandRank::HighCard,
    }
}

fn is_flush(hand: &CardVec) -> bool {
    let first = hand[0];
    hand.iter().all(|c| c.suit == first.suit)
}

fn is_straight(hand: &CardVec) -> bool {
    let mut is_straight = true;

    hand.iter().enumerate().for_each(|(i, c)| {
        //Wheel case
        if i == 3 && is_straight && c.rank == Rank::Five && hand[i + 1].rank == Rank::Ace {
            ;
        } else if is_straight && i < 4 {
            is_straight = c.rank.val() + 1 == hand[i + 1].rank.val();
        }
    });

    is_straight
}
// TODO: There's a way to do this with bitshifting which is probably a bit faster
fn get_buckets(hand: &CardVec) -> [u8; 4] {
    let mut buckets = [0, 0, 0, 0];
    let mut acc = 1;

    hand.iter().enumerate().for_each(|(i, c)| {
        if i < 4 && c.rank == hand[i + 1].rank {
            acc += 1;
        } else {
            buckets[acc - 1] += 1;
            acc = 1;
        }
    });

    buckets
}

pub fn get_kickers(hand: &CardVec, hand_rank: HandRank) -> RankVec {
    // Don't really want to clone here, but also have to enforce sortedness... could optimize by
    // assuming caller provides it sorted, but that doesn't sound great either...
    let mut hand = hand.clone();
    hand.sort();

    match hand_rank {
        HandRank::HighCard | HandRank::Flush => get_unconnected_kickers(&hand),
        HandRank::Straight | HandRank::StraightFlush => get_straight_kickers(&hand),
        _ => get_paired_kickers(&hand, hand_rank),
    }
}

fn get_unconnected_kickers(hand: &CardVec) -> RankVec {
    hand.iter().rev().map(|card| card.rank).collect()
}

fn get_straight_kickers(hand: &CardVec) -> RankVec {
    let first = hand[4].rank;
    let second = hand[3].rank;

    if first == Rank::Ace && second == Rank::Five {
        vec![second]
    } else {
        vec![first]
    }
}

// TODO: There's probably a better way to do this, but on the other hand, it's kinda cool
fn get_paired_kickers(hand: &CardVec, hand_rank: HandRank) -> RankVec {
    let mut kickers = VecDeque::new();
    let mut acc = 1;
    let mut trips_seen = false;
    let mut pair_seen = false;

    // Enumerate before Reversing for fun and profit
    hand.iter().enumerate().rev().for_each(|(i, c)| {
        if i > 0 && c.rank == hand[i - 1].rank {
            acc += 1;
        } else {
            match acc {
                1 => kickers.push_back(c.rank),
                2 if hand_rank == HandRank::Pair => kickers.push_front(c.rank),
                2 if hand_rank == HandRank::TwoPair && !pair_seen => {
                    kickers.push_front(c.rank);
                    pair_seen = true;
                }
                2 if hand_rank == HandRank::TwoPair && pair_seen => kickers.insert(1, c.rank),
                2 if hand_rank == HandRank::FullHouse && !trips_seen => {
                    kickers.push_front(c.rank);
                    pair_seen = true;
                }
                2 if hand_rank == HandRank::FullHouse && trips_seen => kickers.push_back(c.rank),
                3 => {
                    kickers.push_front(c.rank);
                    trips_seen = true;
                }
                4 => kickers.push_front(c.rank),
                _ => (),
            };
            acc = 1;
        }
    });

    kickers.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;

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

}
