use rand::{Rng, thread_rng};

use card::*;
use deck::*;
use hand::*;

pub struct GameState<'a> {
    pub players: Vec<Player>,
    pub pot: Option<Pot<'a>>,
    pub sidepots: Option<Vec<Pot<'a>>>,
    pub deck: Option<Deck>,
    pub button: Option<&'a Player>,
    pub blinds: Blinds,
    pub action: Option<&'a Player>,
    pub current_bet: Option<u32>,
    pub board: Option<CardVec>,
    pub street: Option<Street>,
    pub hand_count: u32,
}

pub struct Player {
    pub id: u32,
    pub name: String,
    pub hand: Option<Hand>,
    pub chips: u32,
}

pub struct Blinds {
    pub bb: u32,
    pub sb: u32,
    pub ante: Option<u32>,
}

pub struct Pot<'a> {
    pub chips: Option<u32>,
    pub participants: Option<Vec<&'a Player>>,
}

pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River,
}

pub fn init_game_state<'a>(mut players: Vec<Player>, blinds: Blinds) -> GameState<'a> {
    if players.len() > 10 {
        panic!("Attmpting to init game with {} players; 10 is the maximum", players.len());
    }
    thread_rng().shuffle(&mut players);

    GameState {
        players,
        pot: None,
        sidepots: None,
        deck: None,
        button: None,
        blinds,
        action: None,
        current_bet: None,
        board: None,
        street: None,
        hand_count: 0,
    }
}

pub fn init_player(id: u32, name: &str, chips: u32) -> Player {
    let name = name.to_string();
    Player {
        id,
        name,
        hand: None,
        chips,
    }
}

pub fn init_blinds(bb: u32, sb: u32, ante: Option<u32>) -> Blinds {
   Blinds {
    bb,
    sb,
    ante,
   } 
}

#[cfg(test)]
mod game_tests{
    use game::*;

    fn get_n_dummy_players(n: u32) -> Vec<Player> {
        (0..n).map(|i| init_player(i, "Dummy", 100)).collect()
    }


    #[test]
    fn it_inits_a_game() {
        let players = get_n_dummy_players(6);
        let blinds = init_blinds(10, 5, None);
        let game = init_game_state(players, blinds);

        assert_eq!(game.hand_count, 0);
        assert_eq!(game.players.len(), 6);
        assert_eq!(game.blinds.bb, 10);
        assert!(game.pot.is_none());
        assert!(game.sidepots.is_none());
        assert!(game.deck.is_none());
        assert!(game.deck.is_none());
        assert!(game.button.is_none());
        assert!(game.action.is_none());
        assert!(game.street.is_none());
        assert!(game.board.is_none());
    }
}
