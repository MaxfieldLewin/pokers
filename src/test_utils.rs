use card::*;

pub fn high_card_hand() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("3", "S"),
        card_from_str("4", "S"),
        card_from_str("5", "S"),
        card_from_str("7", "S"),
    ]
}

pub fn high_card_kickers() -> RankVec {
    vec![Rank::Seven, Rank::Five, Rank::Four, Rank::Three, Rank::Two]
}
// 8 high, five kicker
pub fn high_card_hand_2() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("3", "S"),
        card_from_str("4", "S"),
        card_from_str("5", "S"),
        card_from_str("8", "S"),
    ]
}
// 8 high, six kicker
pub fn high_card_hand_3() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("3", "S"),
        card_from_str("4", "S"),
        card_from_str("6", "S"),
        card_from_str("8", "S"),
    ]
}
// 8 high, six kicker, different suits
pub fn high_card_hand_4() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("3", "H"),
        card_from_str("4", "S"),
        card_from_str("6", "S"),
        card_from_str("8", "S"),
    ]
}
// Pair is lower than kickers
pub fn pair_hand() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("4", "S"),
        card_from_str("2", "S"),
        card_from_str("3", "S"),
        card_from_str("5", "S"),
    ]
}
pub fn pair_kickers() -> RankVec {
    vec![Rank::Two, Rank::Five, Rank::Four, Rank::Three]
}

// Pair is between kickers
pub fn pair_hand_2() -> CardVec {
    vec![
        card_from_str("5", "D"),
        card_from_str("6", "S"),
        card_from_str("2", "S"),
        card_from_str("3", "S"),
        card_from_str("5", "S"),
    ]
}

pub fn pair_2_kickers() -> RankVec {
    vec![Rank::Five, Rank::Six, Rank::Three, Rank::Two]
}

// Pair is higher than kickers
pub fn pair_hand_3() -> CardVec {
    vec![
        card_from_str("5", "D"),
        card_from_str("4", "S"),
        card_from_str("2", "S"),
        card_from_str("3", "S"),
        card_from_str("5", "S"),
    ]
}

pub fn pair_3_kickers() -> RankVec {
    vec![Rank::Five, Rank::Four, Rank::Three, Rank::Two]
}
// the lowliest possible two pair hand
pub fn two_pair_hand() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("3", "S"),
        card_from_str("3", "D"),
        card_from_str("2", "S"),
        card_from_str("4", "S"),
    ]
}
pub fn two_pair_kickers() -> RankVec {
    vec![Rank::Three, Rank::Two, Rank::Four]
}

// aces and threes with a four kicker
pub fn two_pair_hand_2() -> CardVec {
    vec![
        card_from_str("3", "S"),
        card_from_str("3", "D"),
        card_from_str("A", "D"),
        card_from_str("A", "C"),
        card_from_str("4", "S"),
    ]
}
// aces and threes with a king kicker
pub fn two_pair_hand_3() -> CardVec {
    vec![
        card_from_str("3", "S"),
        card_from_str("3", "D"),
        card_from_str("A", "D"),
        card_from_str("A", "C"),
        card_from_str("K", "S"),
    ]
}
pub fn three_of_a_kind_hand() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("2", "S"),
        card_from_str("2", "H"),
        card_from_str("3", "D"),
        card_from_str("4", "S"),
    ]
}
pub fn three_of_a_kind_kickers() -> RankVec {
    vec![Rank::Two, Rank::Four, Rank::Three]
}
pub fn three_of_a_kind_hand_2() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("3", "S"),
        card_from_str("4", "H"),
        card_from_str("4", "D"),
        card_from_str("4", "S"),
    ]
}
pub fn three_of_a_kind_hand_3() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("A", "S"),
        card_from_str("4", "H"),
        card_from_str("4", "D"),
        card_from_str("4", "S"),
    ]
}
pub fn straight_hand() -> CardVec {
    vec![
        card_from_str("3", "D"),
        card_from_str("2", "S"),
        card_from_str("5", "S"),
        card_from_str("4", "H"),
        card_from_str("6", "D"),
    ]
}
pub fn straight_kickers() -> RankVec {
    vec![Rank::Six]
}
pub fn wheel_straight_hand() -> CardVec {
    vec![
        card_from_str("3", "D"),
        card_from_str("2", "S"),
        card_from_str("4", "H"),
        card_from_str("A", "D"),
        card_from_str("5", "S"),
    ]
}
pub fn wheel_straight_kickers() -> RankVec {
    vec![Rank::Five]
}

pub fn flush_hand() -> CardVec {
    vec![
        card_from_str("2", "S"),
        card_from_str("3", "S"),
        card_from_str("4", "S"),
        card_from_str("5", "S"),
        card_from_str("7", "S"),
    ]
}
pub fn flush_kickers() -> RankVec {
    vec![Rank::Seven, Rank::Five, Rank::Four, Rank::Three, Rank::Two]
}
pub fn full_house_hand() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("2", "S"),
        card_from_str("3", "D"),
        card_from_str("3", "S"),
        card_from_str("2", "H"),
    ]
}
pub fn full_house_kickers() -> RankVec {
    vec![Rank::Two, Rank::Three]
}
pub fn four_of_a_kind_hand() -> CardVec {
    vec![
        card_from_str("2", "D"),
        card_from_str("2", "H"),
        card_from_str("2", "C"),
        card_from_str("3", "D"),
        card_from_str("2", "S"),
    ]
}
pub fn four_of_a_kind_kickers() -> RankVec {
    vec![Rank::Two, Rank::Three]
}

pub fn straight_flush_hand() -> CardVec {
    vec![
        card_from_str("2", "S"),
        card_from_str("3", "S"),
        card_from_str("5", "S"),
        card_from_str("6", "S"),
        card_from_str("4", "S"),
    ]
}
pub fn straight_flush_kickers() -> RankVec {
    vec![Rank::Six]
}

pub fn wheel_straight_flush_hand() -> CardVec {
    vec![
        card_from_str("2", "S"),
        card_from_str("3", "S"),
        card_from_str("5", "S"),
        card_from_str("A", "S"),
        card_from_str("4", "S"),
    ]
}
pub fn wheel_straight_flush_kickers() -> RankVec {
    vec![Rank::Five]
}

