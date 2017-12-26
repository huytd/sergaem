use ws::Result;

use engine::network::ServerHandler;

impl ServerHandler {
    pub fn processing_commands(&mut self, msg: &str) -> Result<()> {
        match &msg[0..3] {
            "LST" => {
                println!("PRELOCK");
                let list = self.game_manager.lock().unwrap().get_games_list();
                println!("AFTERLOCK");
                let mut s = String::from("LST#");
                for i in list {
                    s += &format!("{},", i);
                }
                println!("Return game list: {}", s);
                self.send_message(&s)
            },
            "JON" => {
                println!("PRE JOIN LOCK");
                let game_id = (&msg[4..4]).parse::<u64>().unwrap_or(1u64);
                let result = true;
                {
                    self.game_manager.lock().unwrap().player_join_game(self.socket.clone(), game_id);
                }
                println!("AFTER JOIN LOCK");
                if result {
                    return self.send_message("JOINED");
                }
                return self.send_error();
            },
            _ => {
                println!("Unknown text message");
                self.send_error()
            }
        }
    }
}
