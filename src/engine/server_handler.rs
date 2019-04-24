use ws::{CloseCode, Sender, Handler, Handshake, Message, Result};
use ws::util::Token;

pub struct ServerHandler {
    pub socket: Sender
}

impl ServerHandler {
    pub fn send_message(&self, msg: &str) -> Result<()> {
        self.socket.send(Message::from(msg))
    }

    pub fn send_error(&self) -> Result<()> {
        self.socket.send(Message::from("ERROR"))
    }
}

impl Handler for ServerHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Client connected");
        // TODO: Network add player
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        let result = match msg.is_text() {
            true => {
                let msg_text = msg.as_text().unwrap();
                self.processing_commands(msg_text)
            },
            false => {
                println!("Unknown message");
                self.send_error()
            }
        };
        result
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
        // TODO: Game manager remove player
        // TODO: Networ remove player
    }
}
