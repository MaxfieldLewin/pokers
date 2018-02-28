use rand::{thread_rng, Rng};

use card::*;
use deck::*;
use hand::*;
use player::*;

pub struct GameState<'a> {
    // TODO: Could use table/seat abstraction instead of raw Player Vec.
    // Will probably be necessary to support MTTs & possibly proper dead button behavior.
    // Maybe use a ring buffer to be fancy
    // TODO: Genericize to support multiple games
    // TODO: Consider putting players in Boxes
    pub players: PlayerVec,
    pub blinds: Blinds,
    pub pot: Pot<'a>,
    pub sidepots: Vec<Pot<'a>>,
    pub deck: Deck,
    // TODO: Ringbuff would let us switch back to player refs
    pub button: usize,
    // pub button: Option<&'a Player>,
    pub small_blind: usize,
    pub big_blind: usize,
    pub action: usize,
    //pub action: Option<&'a Player>,
    pub current_bet: Option<u32>,
    pub board: CardVec,
    pub street: Street, 
    pub hand_count: u32,
}

impl<'a> GameState<'a>{
    // Init Fns
    pub fn init_round(&mut self) {
        // Init turn: new empty pot, new deck, increment hand count,
        self.pot = init_pot();
        self.deck = init_shuffled_deck();
        self.hand_count += 1;
        self.street = Street::PreFlop;
        self.rotate_button();
    }

    // One function to both rotate button and calc sb/bb/action as they are order dependant
    pub fn rotate_button(&mut self) {
        self.button = match self.button {
            n if n == self.players.len() - 1 => 0,
            _ => self.button + 1,
        };

        self.small_blind = match self.button {
            n if n == self.players.len() - 1 => 0,
            _ => self.button + 1,
        };
 
        self.big_blind = match self.small_blind {
            n if n == self.players.len() - 1 => 0,
            _ => self.small_blind + 1,
        };

        self.action = match self.big_blind {
            n if n == self.players.len() - 1 => 0,
            _ => self.big_blind + 1,
        };
    }

    // Ugh
    pub fn deal_hands(&mut self) {
        for i in 0..self.players.len() {
            self.players[i].hand = Some(init_hand(self.deck.deal_cards(2)));

            match i {
                n if n == self.big_blind => {
                    self.pot.chips += self.players[n].give_chips(self.blinds.bb);
                },
                n if n == self.small_blind => {
                    self.pot.chips += self.players[n].give_chips(self.blinds.sb);
                },
                _ => {}
            }
        }
    }
    
    // Progress game as state machine
    pub fn step(&mut self) {
        match self.street {
            Street::PreFlop => {
                
            },
            Street::Flop => {
            },
            Street::Turn => {
            },
            Street::River => {
            },
            _ => (),
        }
    }

    pub fn end_round(&mut self) {}

    // Utils 
    pub fn game_continuing(&self) -> bool {
        // TODO: Include check that remaining players have any chips
        self.players.len() > 1
    }

    pub fn round_continuing(&self) -> bool {
        self.pot.participants.len() > 1 && self.street != Street::Showdown
    }
}

pub struct Blinds {
    pub sb: u32,
    pub bb: u32,
    pub ante: Option<u32>,
}

pub struct Pot<'a> {
    pub chips: u32,
    pub participants: Vec<&'a Player>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

pub fn init_game_state<'a>(mut players: Vec<Player>, blinds: Blinds) -> GameState<'a> {
    let player_count = players.len();
    if player_count > 10 || player_count < 2 {
        panic!(
            "Attmpting to init game with {} players; must be between 2 and 10",
            players.len()
        );
    }

    // TODO: Separate player ordering logic
    thread_rng().shuffle(&mut players);

    GameState {
        players,
        blinds,
        pot: init_pot(),
        sidepots: vec![],
        deck: init_deck(),
        // Bogus values
        button: 0,
        small_blind: 0,
        big_blind: 0,
        action: 0,
        current_bet: None,
        board: vec![],
        street: Street::PreFlop,
        hand_count: 0,
    }
}

pub fn init_blinds(sb: u32, bb: u32, ante: Option<u32>) -> Blinds {
    Blinds { sb, bb, ante }
}

pub fn init_pot<'a>() -> Pot<'a> {
    Pot {
        participants: vec![],
        chips: 0,
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    fn get_n_dummy_players(n: u32) -> Vec<Player> {
        (0..n).map(|i| init_player(i, "Dummy", 100)).collect()
    }

    #[test]
    fn it_inits_a_game() {
        let players = get_n_dummy_players(6);
        let blinds = init_blinds(5, 10, None);
        let game = init_game_state(players, blinds);

        assert_eq!(game.hand_count, 0);
        assert_eq!(game.players.len(), 6);
        assert_eq!(game.blinds.bb, 10);
        assert_eq!(game.pot.chips, 0);
        assert_eq!(game.pot.participants.len(), 0);
        assert_eq!(game.sidepots.len(), 0);
        assert_eq!(game.button, 0);
        assert_eq!(game.action, 3);
        assert_eq!(game.street, Street::PreFlop);
        assert_eq!(game.board.len(), 0);
    }

    // Could re-write to check error messages using std::panic::catch_unwind
    #[test]
    #[should_panic]
    fn it_enforces_10_player_maximum_when_initing_a_game() {
        let players = get_n_dummy_players(11);
        let blinds = init_blinds(5, 10, None);
        init_game_state(players, blinds);
    }

    #[test]
    #[should_panic]
    fn it_enforces_2_player_minimum_when_initing_a_game() {
        let players = get_n_dummy_players(1);
        let blinds = init_blinds(5, 10, None);
        init_game_state(players, blinds);
    }

}
