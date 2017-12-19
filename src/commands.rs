use ws::Result;

use network::ServerHandler;

impl ServerHandler {
    pub fn processing_commands(&mut self, msg: &str) -> Result<()> {
        match &msg[0..3] {
            "LST" => {
                let list = self.game_manager.borrow().get_games_list();
                let mut s = String::from("LST#");
                for i in list {
                    s += &format!("{},", i);
                }
                println!("Return game list: {}", s);
                self.send_message(&s)
            },
            "JON" => {
                let game_id = (&msg[4..4]).parse::<u64>().unwrap_or(1u64);
                let mut games = self.game_manager.borrow_mut();
                let result = games.player_join_game(self.socket.clone(), game_id);
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
