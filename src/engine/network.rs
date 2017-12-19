use std::rc::Rc;
use std::cell::RefCell;
use ws::{CloseCode, Sender, Handler, Handshake, Message, Result};
use ws::util::Token;

use engine::game_manager::GameManagerRef;

const MAX_USERS_ALLOWED: usize = 2000;

pub type NetworkManagerRef = Rc<RefCell<NetworkManager>>;

pub struct NetworkManager {
    pub clients: Vec<Sender>
}

impl NetworkManager {
    pub fn new() -> NetworkManager {
        NetworkManager {
            clients: Vec::with_capacity(MAX_USERS_ALLOWED)
        }
    }

    pub fn new_ref() -> NetworkManagerRef {
        Rc::new(RefCell::new(NetworkManager::new()))
    }

    pub fn get_total_clients(&self) -> usize {
        self.clients.len()
    }

    pub fn add_client(&mut self, client: Sender) -> bool {
        if self.clients.len() <= MAX_USERS_ALLOWED {
            self.clients.push(client);
            return true;
        }
        false
    }

    pub fn remove_client(&mut self, token: Token) {
        let mut found: i32 = -1;
        let mut index = 0;
        for client in self.clients.iter() {
            if client.token().eq(&token) {
                found = index;
                break;
            }
            index += 1;
        }
        if found != -1 {
            self.clients.remove(found as usize);
        }
    }
}

pub struct ServerHandler {
    pub socket: Sender,
    pub manager: NetworkManagerRef,
    pub game_manager: GameManagerRef
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
        self.manager.borrow_mut().add_client(self.socket.clone());
        println!("Total clients: {}", self.manager.borrow().get_total_clients());
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
        self.game_manager.borrow_mut().remove_player(self.socket.token());
        self.manager.borrow_mut().remove_client(self.socket.token());
        println!("Total clients: {}", self.manager.borrow().get_total_clients());
    }
}
