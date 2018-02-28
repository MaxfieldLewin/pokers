use hand::Hand;

pub struct Player {
    pub id: u32,
    pub name: String,
    pub hand: Option<Hand>,
    pub chips: u32,
}

pub type PlayerVec = Vec<Player>;

impl Player {
    pub fn give_chips(&mut self, amount: u32) -> u32 {
        self.chips -= amount;
        amount
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

pub fn init_players(num_players: u32, user_player: bool, chips: u32) -> PlayerVec {
    (0..num_players).map(|i| {
        let mut name = "Player ".to_string();
        name.push_str(&i.to_string());
        init_player(i, &name, chips)
    }).collect()
}


