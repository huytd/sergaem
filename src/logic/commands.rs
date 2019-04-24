use ws::Result;

use engine::server_handler::ServerHandler;

impl ServerHandler {
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
