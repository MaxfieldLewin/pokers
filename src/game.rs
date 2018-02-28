use player::*;
use gamestate::*;

// The top level game loop. 
pub fn play_game(num_players: u32, user_player: bool, starting_stack: u32, sb: u32, bb: u32, ante: Option<u32>) {

    let players = init_players(num_players, user_player, starting_stack);
    let blinds = init_blinds(sb, bb, ante);
    let mut game_state = init_game_state(players, blinds);

    while game_state.game_continuing() {
        game_state.init_round();
        game_state.deal_hands();
        
        while game_state.round_continuing() {
            game_state.step();
        }

        game_state.end_round();
    }
}


