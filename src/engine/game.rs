use std::time::{Duration, SystemTime};
use std::thread;
use std::sync::mpsc;
use ws::Sender;
use ws::util::Token;

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
            is_started: false
        }
    }

    pub fn start_game(&self) {
        // For now, there's no way to stop the game loop once it started
        // TODO: Bring tx to the struct level, where we can accessit and 
        // send stop signal whenever we want.
        let (tx, rx) = mpsc::channel();
        tx.send(self.is_started);
        thread::spawn(move || {
            let mut is_stopped = false;
            let mut last_frame = SystemTime::now();
            while !is_stopped {
                let current_time = SystemTime::now();
                let frame_count = current_time.duration_since(last_frame).unwrap();
                if frame_count.ge(&Duration::from_millis(16)) {
                    // It's 60 fps here
                    last_frame = current_time;
                }
                is_stopped = rx.recv().unwrap_or(false);
            }
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
}
