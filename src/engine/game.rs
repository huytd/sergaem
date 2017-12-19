use ws::Sender;
use ws::util::Token;

const MAX_PLAYERS_ALLOWED: usize = 100;

#[derive(Clone)]
pub struct Game {
    pub id: u64,
    pub players: Vec<Sender>
}

impl Game {
    pub fn new(game_id: u64) -> Game {
        Game {
            id: game_id,
            players: Vec::with_capacity(MAX_PLAYERS_ALLOWED)
        }
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
