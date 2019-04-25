use ws::{CloseCode, Sender, Handler, Handshake, Message, Result};
use ws::util::Token;
use std::sync::mpsc;

const MAX_USERS_ALLOWED: usize = 2000;

pub struct NetworkManager {
    pub clients: Vec<Sender>,
    receiver: mpsc::Receiver<String>
}

impl NetworkManager {
    pub fn new(receiver: mpsc::Receiver<String>) -> NetworkManager {
        NetworkManager {
            clients: Vec::with_capacity(MAX_USERS_ALLOWED),
            receiver: receiver
        }
    }

    pub fn listen(&self) {
        loop {
            if let Ok(msg) = self.receiver.recv() {
                println!("DBG::NET GOT DATA {}", msg);
            }
        }
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
