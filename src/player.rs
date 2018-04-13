use std::rc::Rc;
use std::cell::RefCell;

use rand::{thread_rng, Rng};
use card::CardVec;
use hand::Hand;

#[derive(Clone, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub hole_cards: Option<CardVec>,
    pub chips: u32,
    pub last_action: Option<PlayerAction>,
    pub in_hand: bool,
    pub all_in: bool,
}

pub type PlayerVec = Vec<Player>;

pub type PlayerId = u32;

impl Player {
    pub fn init_for_round(&mut self) {
        self.in_hand = true;
        self.last_action = None;
        self.hole_cards = None;
        self.all_in = false;
        println!(
            "Player {} starting round with {} chips",
            self.id, self.chips
        );
    }

    pub fn bet(&mut self, amount: u32) -> PlayerAction {
        let bet = self.give_chips(amount);
        self.last_action = Some(PlayerAction::Bet(bet));
        PlayerAction::Bet(bet)
    }

    pub fn call(&mut self, amount: u32) -> PlayerAction {
        let call = self.give_chips(amount);
        self.last_action = Some(PlayerAction::Call(call));
        PlayerAction::Call(call)
    }

    pub fn raise(&mut self, amount: u32) -> PlayerAction {
        let raise = self.give_chips(amount);
        self.last_action = Some(PlayerAction::Raise(raise));
        PlayerAction::Raise(raise)
    }

    pub fn check(&mut self) -> PlayerAction {
        self.last_action = Some(PlayerAction::Check);
        PlayerAction::Check
    }

    pub fn fold(&mut self) -> PlayerAction {
        self.last_action = Some(PlayerAction::Fold);
        self.in_hand = false;
        PlayerAction::Fold
    }

    pub fn give_blinds(&mut self, blind_amount: u32) -> PlayerAction {
        self.bet(blind_amount)
    }

    pub fn give_chips(&mut self, amount: u32) -> u32 {
        if amount >= self.chips {
            let amount = self.chips;
            self.chips = 0;
            self.all_in = true;
            amount
        } else {
            self.chips -= amount;
            amount
        }
    }

    pub fn receive_chips(&mut self, amount: u32) {
        self.chips += amount;
    }

    pub fn announce_action(&mut self, current_bet: Option<u32>, minbet: u32) -> PlayerAction {
        let last_bet = self.get_last_bet_amount();

        let mut allowed_actions = match current_bet {
            Some(n) if n >= self.chips => vec![PlayerAction::Call(n), PlayerAction::Fold],
            Some(n) if n > last_bet => vec![
                PlayerAction::Call(n),
                PlayerAction::Raise(n * 2),
                PlayerAction::Fold,
            ],
            Some(n) if n == last_bet && last_bet > 0 => {
                vec![PlayerAction::Raise(n * 2), PlayerAction::Check]
            }
            _ => vec![PlayerAction::Check, PlayerAction::Bet(minbet)],
        };

        //allowed_actions.push(PlayerAction::Fold);

        match thread_rng().choose(&allowed_actions).unwrap() {
            &PlayerAction::Check => self.check(),
            &PlayerAction::Fold => self.fold(),
            &PlayerAction::Bet(n) => self.bet(n),
            &PlayerAction::Call(n) => self.call(n),
            &PlayerAction::Raise(n) => self.raise(n),
        }
    }

    fn get_last_bet_amount(&mut self) -> u32 {
        match self.last_action {
            Some(PlayerAction::Bet(n))
            | Some(PlayerAction::Call(n))
            | Some(PlayerAction::Raise(n)) => n,
            _ => 0,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PlayerAction {
    Bet(u32),
    Call(u32),
    Raise(u32),
    Check,
    Fold,
}

pub fn init_player(id: u32, name: &str, chips: u32) -> Player {
    let name = name.to_string();
    Player {
        id,
        name,
        hole_cards: None,
        chips,
        last_action: None,
        in_hand: false,
        all_in: false,
    }
}

pub fn init_players(num_players: u32, user_player: bool, chips: u32) -> PlayerVec {
    (0..num_players)
        .map(|i| {
            let mut name = "Player ".to_string();
            name.push_str(&i.to_string());
            init_player(i, &name, chips)
        })
        .collect()
}


pub fn init_shared_players(num_players: usize, chips: u32) -> Vec<Rc<RefCell<Player>>> {
    (0..num_players as u32)
        .map(|i| {
            let mut name = "Player ".to_string();
            name.push_str(&i.to_string());
            let mut player = init_player(i, &name, chips);
            Rc::new(RefCell::new(player))
        })
        .collect()
}
