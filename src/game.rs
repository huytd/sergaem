use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Game {
    pub id: u64,
}

impl Game {
    pub fn new(game_id: u64) -> Game {
        Game {
            id: game_id
        }
    }
}
