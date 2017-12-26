extern crate crossbeam;

use std::time::{Duration, SystemTime};
use ws::{Sender, Message};
use ws::util::Token;
use logic::game_logic::GameLogic;

const MAX_PLAYERS_ALLOWED: usize = 100;

#[derive(Clone)]
pub struct Game {
    pub id: u64,
    pub players: Vec<Sender>,
    pub is_started: bool
}

impl Game {
    pub fn new(game_id: u64) -> Game {
        Game {
            id: game_id,
            players: Vec::with_capacity(MAX_PLAYERS_ALLOWED),
            is_started: true
        }
    }

    pub fn start_game(&self) {
        crossbeam::scope(|scope| {
            scope.spawn(move || {
                let mut last_frame = SystemTime::now();
                let mut game_logic = GameLogic::new(self.clone());
                game_logic.init();
                while self.is_started {
                    let current_time = SystemTime::now();
                    let frame_count = current_time.duration_since(last_frame).unwrap();
                    if frame_count.ge(&Duration::from_millis(16)) {
                        // It's 60 fps here
                        game_logic.update();
                        last_frame = current_time;
                    }
                }
            });
        });
    }

    pub fn kick(&mut self, player_token: Token) {
        let mut index = 0;
        let mut found: i32 = -1;
        for p in self.players.iter() {
            if p.token().eq(&player_token) {
                found = index;
                break;
            }
            index += 1;
        }
        if found != -1 {
            (&mut self.players).remove(found as usize);
        }
        println!("TOTAL PLAYERS LEFT IN ROOM: {}", self.players.len());
    }

    pub fn join(&mut self, player: Sender) -> bool {
        if self.players.len() <= MAX_PLAYERS_ALLOWED {
            self.players.push(player);
            println!("TOTAL PLAYERS IN ROOM: {}", self.players.len());
            return true;
        }
        false
    }

    pub fn broadcast(&self, msg: &str) {
        for player in self.players.iter() {
            player.send(Message::from(msg));
        }
    }
}
