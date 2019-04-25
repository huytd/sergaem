use ws::{CloseCode, Sender, Handler, Handshake, Message, Result};
use ws::util::Token;
use crossbeam::channel as channel;

pub struct ClientHandler {
    pub socket: Sender,
    pub game_manager_bus: channel::Sender<String>,
    pub network_manager_bus: channel::Sender<String>
}

impl ClientHandler {
    pub fn notify(&self, msg: &str) {
        self.game_manager_bus.send(msg.to_string());
        self.network_manager_bus.send(msg.to_string());
    }

    pub fn send_message(&self, msg: &str) -> Result<()> {
        self.socket.send(Message::from(msg))
    }

    pub fn send_error(&self) -> Result<()> {
        self.socket.send(Message::from("ERROR"))
    }
}

impl Handler for ClientHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Client connected");
        // TODO: Network add player
        self.notify("PLAYER-CONNECT");
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
