use hand::Hand;

pub struct Player {
    pub id: u32,
    pub name: String,
    pub hand: Option<Hand>,
    pub chips: u32,
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
