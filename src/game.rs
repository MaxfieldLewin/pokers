pub struct GameState {
    pub players: Vec<Player>,
    pub pot: Pot,
    pub sidepots: Vec<Pot>,
    pub deck: Deck,
    pub button: &Player,
    pub blinds: Blinds,
    pub action: &Player,
    pub current_bet: Option(u32)
    pub street: Street,
}

pub struct Player {
    pub id: u32,
    pub name: &str,
    pub hand: Option(Hand),
    pub chips: u32,
}

pub struct Pot {
    pub chips: u32,
    pub participants: Vec<&Player>,
}

pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River,
}
