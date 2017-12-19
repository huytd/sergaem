use engine::game::Game;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ws::Sender;
use ws::util::Token;

pub type GameManagerRef = Rc<RefCell<GameManager>>;

const MAX_GAMES_ALLOWED: usize = 500;

pub struct GameManager {
    pub games: HashMap<u64, Game>
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager {
            games: HashMap::with_capacity(MAX_GAMES_ALLOWED)
        }
    }

    pub fn new_ref() -> GameManagerRef {
        Rc::new(RefCell::new(GameManager::new()))
    }

    pub fn get_next_id(&self) -> u64 {
        let mut max: u64 = 0;
        for key in self.games.keys() {
            if *key > max {
                max = *key;
            }
        }
        max + 1 
    }

    pub fn remove_player(&mut self, player_token: Token) {
        for (_, mut game) in &mut self.games {
            game.kick(player_token);
        }
    }

    pub fn create_game(&mut self) -> bool {
        let game = Game::new(self.get_next_id());
        if self.games.len() <= MAX_GAMES_ALLOWED {
            self.games.insert(game.id, game.clone());
            return true;
        }
        false
    }

    pub fn start_game(&self, game_id: u64) {
        if let Some(game) = self.games.get(&game_id) {
            game.start_game();
        }
    }

    pub fn get_games_list(&self) -> Vec<u64> {
        let mut result: Vec<u64> = vec![];
        for key in self.games.keys() {
            result.push(*key);
        }
        result
    }

    pub fn player_join_game(&mut self, player: Sender, game_id: u64) -> bool {
        if let Some(game) = self.games.get_mut(&game_id) {
            return game.join(player);
        }
        false
    }
}
