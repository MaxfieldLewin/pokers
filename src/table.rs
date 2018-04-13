use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use player::*;

pub type SeatId = u32;
pub type SeatedPlayer = Rc<RefCell<Player>>;
pub type Seat = Option<SeatedPlayer>;

pub struct Table {
    seats: Vec<SeatId>,
    seat_count: usize,
    seat_map: HashMap<SeatId, Seat>,
    button: SeatId,
    action: SeatId,
}

impl Table {
}

pub fn init_table(seat_count: usize, chips: u32) -> Table {
    let mut players = init_shared_players(seat_count, chips);

    let mut seats = vec![];
    let mut seat_map = HashMap::new();
    let mut i = 0;
    for p in players {
        seats.push(i);

        seat_map.insert(i, Some(p.clone()));

        i += 1;
    }


    Table {
       seats,
       seat_count,
       seat_map,
       button: 0,
       action: 0,
    }
}
