use card::CardVec;
use hand::Hand;

pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub hole_cards: Option<CardVec>,
    pub hand: Option<Hand>,
    pub chips: u32,
    pub last_action: Option<PlayerAction>,
    pub in_hand: bool,
}

pub type PlayerVec = Vec<Player>;

pub type PlayerId = u32;

impl Player {
    pub fn make_bet(&mut self, amount: u32) -> u32 {
        self.chips -= amount;
        self.last_action = Some(PlayerAction::Bet(amount));

        amount
    }

    pub fn receive_chips(&mut self, amount: u32) {
        self.chips += amount;
    }

    pub fn announce_action(&mut self) -> PlayerAction {
        PlayerAction::Check
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
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
        hand: None,
        chips,
        last_action: None,
        in_hand: false,
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
