extern crate pokers;

use pokers::deck;
use pokers::hand_rankings;

fn main() {
    let mut d = deck::create_shuffled_deck();
    let mut h = d.deal_cards(5);
    h.sort();
    println!("Hand: {:?}", h);

    let r = hand_rankings::rank_hand(&h);
    println!("Rank: {:?}", r);
}
