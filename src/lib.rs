mod card;
//mod deck;
//mod hand;
//mod hand_rankings;

#[cfg(test)]
mod tests {
    use card;
    #[test]
    fn it_formats_cards() {
        let ace_of_spades = card::Card {
            suit: card::Suit::Spades,
            rank: card::Rank::Ace,
        };
        
        assert_eq!("A of ♠", format!("{}", ace_of_spades));
    }
}
