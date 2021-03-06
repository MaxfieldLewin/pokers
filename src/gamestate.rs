use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};
use std::iter::Filter;

use card::*;
use deck::*;
use hand::*;
use player::*;

#[derive(Debug)]
pub struct GameState {
    // TODO: Could use table/seat abstraction instead of raw Player Vec.
    // Will probably be necessary to support MTTs & possibly proper dead button behavior.
    // Maybe use a ring buffer to be fancy.
    // TODO: Genericize to support multiple games.
    // TODO: Consider putting players in Boxes
    pub players: PlayerVec,
    pub blinds: Blinds,
    pub pot: Pot,
    pub sidepots: Vec<Pot>,
    pub deck: Deck,
    pub button: usize,
    pub small_blind: usize,
    pub big_blind: usize,
    pub player_to_act: usize,
    pub current_bet: Option<u32>,
    pub board: CardVec,
    pub street: Street,
    pub hand_count: u32,
}

#[derive(Debug)]
pub struct Blinds {
    pub sb: u32,
    pub bb: u32,
    pub ante: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Pot {
    pub chips: u32,
    pub participants: HashSet<PlayerId>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}
impl GameState {
    pub fn play(&mut self) {
        // The top level game loop, abstracting one hand (termed round in the code) of poker.
        // Loops while there is more than 1 player at the table, and any player with more than 0 chips
        println!("Game starting");
        while self.game_continuing() {
            //plumbing
            println!("Round {} starting", self.hand_count);
            self.init_round();

            // game logic

            // round setup
            self.rotate_button();
            self.take_blinds();
            self.deal_hands();

            // playing round, until the showdown or one player remaining
            'round: while self.round_continuing() {
                self.advance_player_to_act();
                if self.is_betting_done() {
                    self.transition_street();
                } else {
                    if self.players[self.player_to_act].all_in {
                        continue 'round;
                    }
                    let current_bet = self.current_bet.clone();
                    let action = self.players[self.player_to_act]
                        .announce_action(current_bet, self.blinds.bb);
                    let id = self.players[self.player_to_act].id;
                    self.apply_action(action, id);
                }
            }

            self.award_pots();
            // plumbing
            self.end_round();
        }

        println!("Game over! Player {} wins", self.players[0].id);
    }

    // Plumbing
    // Reset ephemeral round state (excluding player idxs), increment hand counter
    fn init_round(&mut self) {
        self.pot = init_pot();
        self.deck = init_shuffled_deck();
        self.hand_count += 1;
        self.street = Street::PreFlop;
        self.sidepots = vec![];
        self.board = vec![];

        for ref mut player in &mut self.players {
            player.init_for_round();
        }
    }

    fn end_round(&mut self) {
        self.players = self.players
            .iter()
            .cloned()
            .filter(|p| p.chips > 0)
            .collect();
    }

    // One function to both rotate button and calc sb/bb/player_to_act as they are order dependant
    // TODO: Make this not terrible.
    // TODO: Handle heads up
    fn rotate_button(&mut self) {
        println!("Rotating button");
        self.button = match self.button {
            n if n + 1 >= self.players.len() => 0,
            _ => self.button + 1,
        };
        println!("Button at idx {}", self.button);

        self.small_blind = match self.button {
            n if n + 1 >= self.players.len() => 0,
            _ => self.button + 1,
        };
        println!("SB at idx {}", self.small_blind);

        self.big_blind = match self.small_blind {
            n if n + 1 >= self.players.len() => 0,
            _ => self.small_blind + 1,
        };
        println!("BB at idx {}", self.big_blind);

        self.player_to_act = match self.big_blind {
            n if n + 1 >= self.players.len() => 0,
            _ => self.big_blind + 1,
        };
        println!("PTA at idx {}", self.player_to_act);
    }

    fn deal_hands(&mut self) {
        for ref mut player in &mut self.players {
            player.hole_cards = Some(self.deck.deal_cards(2));
            println!("Dealt {:?} to player {}", player.hole_cards, player.id);
        }
    }

    // TODO: Make blinds occur as betting actions
    fn take_blinds(&mut self) {
        let action = self.players[self.small_blind].give_blinds(self.blinds.sb);
        let id = self.players[self.small_blind].id;
        self.apply_action(action, id);

        let action = self.players[self.big_blind].give_blinds(self.blinds.bb);
        let id = self.players[self.big_blind].id;
        self.apply_action(action, id);
    }

    // More of this bad pattern
    fn advance_player_to_act(&mut self) {
        loop {
            self.player_to_act += 1;
            if self.player_to_act >= self.players.len() {
                self.player_to_act = 0;
            }

            if self.players[self.player_to_act].in_hand {
                break;
            }
        }
    }

    fn transition_street(&mut self) {
        // plumbing
        for ref mut player in &mut self.players {
            player.last_action = None;
        }

        // gamelogic
        self.current_bet = None;
        self.reset_player_to_act();
        match self.street {
            Street::PreFlop => {
                self.board.append(&mut self.deck.deal_cards(3));
                self.street = Street::Flop;
                println!("Flop: {:?}", self.board);
            }
            Street::Flop => {
                self.board.append(&mut self.deck.deal_cards(1));
                self.street = Street::Turn;
                println!("Turn: {:?}", self.board);
            }
            Street::Turn => {
                self.board.append(&mut self.deck.deal_cards(1));
                self.street = Street::River;
                println!("River: {:?}", self.board);
            }
            Street::River => {
                self.street = Street::Showdown;
                println!("Showdown");
            }
            Street::Showdown => {
                panic!("This ain't suppposed to happen");
            }
        }
    }

    fn reset_player_to_act(&mut self) {
        self.player_to_act = self.button + 1;

        loop {
            if self.player_to_act >= self.players.len() {
                self.player_to_act = 0;
            }

            if self.players[self.player_to_act].in_hand {
                break;
            }

            self.player_to_act += 1;
        }
    }

    fn apply_action(&mut self, action: PlayerAction, id: u32) {
        match action {
            PlayerAction::Bet(bet) => {
                println!("Player {} bets {} chips", id, bet);
                self.pot.chips += bet;
                self.pot.participants.insert(id);
                self.current_bet = Some(bet);
            }
            PlayerAction::Raise(bet) => {
                println!("Player {} raises to {} chips", id, bet);
                self.pot.chips += bet;
                self.pot.participants.insert(id);
                self.current_bet = Some(bet);
            }
            PlayerAction::Call(bet) => {
                println!("Player {} calls {} chips", id, bet);
                self.pot.chips += bet;
                self.pot.participants.insert(id);
                self.current_bet = Some(bet);
            }
            PlayerAction::Check => {
                println!("Player {} checks", id);
            }
            PlayerAction::Fold => {
                println!("Player {} folds", id);
            }
        }
    }

    fn award_pots(&mut self) {
        if Street::Showdown == self.street {
            // Whats the idiomatic rust way to create a vec from a vec and an item?
            self.sidepots.push(self.pot.clone());
            for mut pot in &self.sidepots {
                let winner_ids = &self.determine_pot_winners(pot.participants.clone());
                println!("Winners are: {:?}", winner_ids);
                let chop = winner_ids.len() as u32;
                let chips = (pot.chips - (pot.chips % chop)) / chop;
                let remainder = pot.chips % chop;
                let remainder_winner = thread_rng().choose(winner_ids).unwrap();

                for id in winner_ids {
                    let (winner_idx, _) = self.players
                        .iter()
                        .enumerate()
                        .filter(|&(i, p)| p.in_hand && p.id == *id)
                        .next()
                        .expect("Award pots: showdown");
                    println!(
                        "Player {} is a winner, receiving {} chips",
                        self.players[winner_idx].id, chips
                    );
                    self.players[winner_idx].receive_chips(chips);
                    if id == remainder_winner {
                        self.players[winner_idx].receive_chips(remainder);
                    }
                }
            }
        } else {
            let (winner_idx, _) = self.players
                .iter()
                .enumerate()
                .filter(|&(i, p)| p.in_hand)
                .next()
                .expect("Award pots: pre-showdown branch, no winner!");
            self.players[winner_idx].receive_chips(self.pot.chips);
        }
    }

    // Utils
    fn game_continuing(&self) -> bool {
        // This redundancy should allow for players to run out of chips but not leave the game
        self.players.len() > 1 && self.num_players_with_chips() > 0
    }

    fn round_continuing(&self) -> bool {
        self.num_hand_participants() > 1 && self.street != Street::Showdown
    }

    // This is pretty damn convoluted
    fn is_betting_done(&mut self) -> bool {
        if self.street == Street::PreFlop && self.big_blind == self.player_to_act
            && self.players[self.big_blind].last_action.unwrap()
                == PlayerAction::Bet(self.blinds.bb)
        {
            println!("Preflop, bb is pta, gs debug: {:?}", self);
            return false;
        }

        if self.players
            .iter()
            .filter(|p| p.in_hand)
            .all(|p| Some(PlayerAction::Check) == p.last_action || p.all_in)
        {
            return true;
        } else if self.current_bet.is_none() {
            return false;
        }

        // Safe to unwrap it because of early return above
        let current_bet = self.current_bet.unwrap();
        self.players
            .iter()
            .filter(|p| p.in_hand)
            .all(|p| match p.last_action {
                Some(PlayerAction::Bet(bet))
                | Some(PlayerAction::Call(bet))
                | Some(PlayerAction::Raise(bet)) if bet == current_bet =>
                {
                    true
                }
                _ => false,
            })
    }

    // yikes
    fn determine_pot_winners(&self, participants: HashSet<PlayerId>) -> Vec<PlayerId> {
        println!("Determining pot winners");
        let mut player_hand_map = HashMap::new();
        let mut hands = vec![];
        let mut board = self.board.clone();
        let mut winners = vec![];

        for id in participants {
            if let Some(player) = self.players
                .iter()
                .filter(|p| p.in_hand && p.id == id)
                .next()
            {
                let mut hole_cards: CardVec = player.hole_cards.clone().unwrap();
                let mut all_cards = board.clone();
                all_cards.append(&mut hole_cards);
                let mut players_best_hand = find_best_hand(all_cards);
                println!("Player {}s best hand: {:?}", id, players_best_hand);
                player_hand_map.insert(id, players_best_hand.clone());
                hands.push(players_best_hand);
            }
        }

        hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let best = hands.iter().last().unwrap();
        player_hand_map.iter().for_each(|(id, hand)| {
            if hand == best {
                winners.push(*id);
            }
        });

        winners
    }

    fn num_players_with_chips(&self) -> u32 {
        self.players.iter().filter(|p| p.chips > 0).count() as u32
    }

    fn num_hand_participants(&self) -> u32 {
        self.players.iter().filter(|p| p.in_hand).count() as u32
    }
}

pub fn init_game_state(mut players: Vec<Player>, blinds: Blinds) -> GameState {
    let player_count = players.len();
    if player_count > 10 || player_count < 2 {
        panic!(
            "Attmpting to init game with {} players; must be between 2 and 10",
            players.len()
        );
    }

    // TODO: Separate player ordering logic
    // wiring off for debug purposes
    // thread_rng().shuffle(&mut players);

    GameState {
        players,
        blinds,
        pot: init_pot(),
        sidepots: vec![],
        deck: init_deck(),
        // Bogus values
        button: 0,
        small_blind: 1,
        big_blind: 2,
        player_to_act: 3,
        current_bet: None,
        board: vec![],
        street: Street::PreFlop,
        hand_count: 0,
    }
}

pub fn init_blinds(sb: u32, bb: u32, ante: Option<u32>) -> Blinds {
    Blinds { sb, bb, ante }
}

pub fn init_pot() -> Pot {
    Pot {
        participants: HashSet::new(),
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
        assert_eq!(game.street, Street::PreFlop);
        assert_eq!(game.board.len(), 0);
    }

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
