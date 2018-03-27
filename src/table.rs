use std::rc::Rc;
use std::cell:RefCell;
use std::collections::HashMap;

use player::*;

pub type SeatId = u32;

pub type Seat = Option<Rc<RefCell<Player>>>>;

pub struct Table {
    players: Vec<Rc<RefCell<Player>>>,
    seats: Vec<SeatId>
    seat_count: usize,
    seat_map: HashMap<SeatId, Seat>,
    button: SeatId,
    action: SeatId,
}
