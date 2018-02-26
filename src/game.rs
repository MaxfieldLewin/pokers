use std::collections::HashMap;
use rand::{Rng, thread_rng};

use card::*;
use deck::*;
use hand::*;

pub struct GameState<'a> {
    pub table: Table,
    pub pot: Option<Pot>,
    pub sidepots: Option<Vec<Pot>>,
    pub deck: Option<Deck>,
    pub button: Option<&'a Seat>,
    pub blinds: Blinds,
    pub action: Option<&'a Seat>,
    pub current_bet: Option<u32>,
    pub board: Option<CardVec>,
    pub street: Option<Street>,
    pub hand_count: u32,
}

pub struct Table {
    pub seat_count: u32,
    pub seat_map: HashMap<u32, Seat>,
}

pub struct Seat {
    pub id: u32,
    pub player: Option<Player>,
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

pub struct Pot {
    pub chips: Option<u32>,
    pub participants: Option<Vec<&'static Player>>,
}

pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River,
}

pub fn init_game_state<'a>(table: Table, blinds: Blinds) -> GameState<'a> {
    GameState {
        table,
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

pub fn init_table(mut players: Vec<Player>, seat_count: u32) -> Table {
    if players.len() > seat_count as usize {
        panic!("Attmpting to init table with {} players and {} seats", players.len(), seat_count);
    }

    thread_rng().shuffle(&mut players);
    let mut seat_map = HashMap::new();
    for i in 0..seat_count {
        if let Some(player) = players.pop() {
            seat_map.insert(i, init_seat(i, Some(player)));
        } else {
            seat_map.insert(i, init_seat(i, None));
        }
    }

    Table {
        seat_count,
        seat_map,
    }
}

pub fn init_seat(id: u32, player: Option<Player>) -> Seat {
    Seat {
        id,
        player,
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
    fn it_inits_a_table() {
        let table = init_table(get_n_dummy_players(6), 6);

        assert_eq!(table.seat_count, 6);
        assert_eq!(table.seat_map.len(), 6);
    }

    #[test]
    fn it_inits_a_game() {
        let table = init_table(get_n_dummy_players(6), 6);
        let blinds = init_blinds(10, 5, None);
        let game = init_game_state(table, blinds);

        assert_eq!(game.hand_count, 0);
        assert_eq!(game.table.seat_count, 6);
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
