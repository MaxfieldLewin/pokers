use gamestate::*;

pub fn play_game(num_players: u32, user_player: bool, sb: u32, bb: u32, ante: Option<u32>) {

    let mut players = init_players(num_players, user_player);
    let mut blinds = init_blinds(sb, bb, ante);
    let mut game_state = init_game_state(players, blinds);

    // TODO: make proper wincon check
    while game_state.game_continuing() {
        // Init turn: new empty pot, new deck, increment hand count, 
        // Reset street, rotate button, set action
        game_state.init_round();
        game_state.deal_round();
        
        while game_state.round_continuing() {
            game_state.step();
        }

        game_state.
    }
}


