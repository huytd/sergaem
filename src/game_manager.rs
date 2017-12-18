use game::Game;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub type GameManagerRef = Rc<RefCell<GameManager>>;
pub type GamesRef = Rc<RefCell<HashMap<u64, Game>>>;

const MAX_GAMES_ALLOWED: usize = 500;

pub struct GameManager {
    pub games: GamesRef
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            games: Rc::new(RefCell::new(HashMap::with_capacity(MAX_GAMES_ALLOWED)))
        }
    }

    pub fn new_ref() -> GameManagerRef {
        Rc::new(RefCell::new(GameManager::new()))
    }

    pub fn get_total_games(&self) -> usize {
        self.games.borrow().len()
    }

    pub fn get_next_id(&self) -> u64 {
        (self.games.borrow().len() + 1) as u64
    }

    pub fn create_game(&mut self) -> Result<u64, u64> {
        let game = Game::new(self.get_next_id());
        if self.games.borrow().len() <= MAX_GAMES_ALLOWED {
            self.games.borrow_mut().insert(game.id, game.clone());
            return Ok(game.id);
        }
        Err(0)
    }

    pub fn get_games_list(&self) -> Vec<u64> {
        let mut result: Vec<u64> = vec![];
        for key in self.games.borrow().keys() {
            result.push(*key);
        }
        result
    }
}
