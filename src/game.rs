use std::collections::HashMap;
use card::*;
use deck::*;
use hand::*;

pub struct GameState {
    pub table: Table,
    pub pot: Option<Pot>,
    pub sidepots: Option<Vec<Pot>>,
    pub deck: Option<Deck>,
    pub button: Option<&'static Seat>,
    pub blinds: Option<Blinds>,
    pub action: Option<&'static Seat>,
    pub current_bet: Option<u32>,
    pub board: Option<CardVec>,
    pub street: Option<Street>,
    pub hand_count: u32,
}

pub struct Table {
    pub seat_count: u32,
    pub seat_map: HashMap<u32, Option<Player>>,
}

pub struct Seat {
    pub seat_id: u32,
    pub player: Option<&'static Player>,
}


pub fn init_game_state(table: Table, blinds: Blinds) -> GameState {
    GameState {
        table,
        pot: None,
        sidepots: None,
        deck: None,
        button: None,
        blinds: Some(blinds),
        action: None,
        current_bet: None,
        board: None,
        street: None,
        hand_count: 0,
    }
}

pub struct Player {
    pub id: u32,
    pub name: &'static str,
    pub hand: Option<Hand>,
    pub chips: u32,
}

pub struct Pot {
    pub chips: Option<u32>,
    pub participants: Option<Vec<&'static Player>>,
}

pub struct Blinds {
    pub little_blind: u32,
    pub big_blind: u32,
    pub ante: Option<u32>,
}

pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River,
}
