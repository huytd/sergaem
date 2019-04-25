use ws::Result;

use engine::client_handler::ClientHandler;

impl ClientHandler {
    pub fn processing_commands(&mut self, msg: &str) -> Result<()> {
        match &msg[0..3] {
            "LST" => {
                return self.send_message("LST")
            },
            "JON" => {
                return self.send_message("JON")
            },
            _ => {
                println!("Unknown text message");
                self.send_error()
            }
        }
    }
}
