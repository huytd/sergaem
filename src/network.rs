use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ws::{CloseCode, Sender, Handler, Handshake, Message, Result};

const MAX_USERS_ALLOWED: usize = 2000;

type NetworkManagerRef = Rc<RefCell<NetworkManager>>;
type ClientRef = Rc<RefCell<HashMap<u64, Sender>>>;

pub struct NetworkManager {
    pub clients: ClientRef
}

impl NetworkManager {
    pub fn new() -> NetworkManager {
        NetworkManager {
            clients: Rc::new(RefCell::new(HashMap::with_capacity(MAX_USERS_ALLOWED)))
        }
    }

    pub fn new_ref() -> NetworkManagerRef {
        Rc::new(RefCell::new(NetworkManager::new()))
    }

    pub fn get_total_clients(&self) -> usize {
        self.clients.borrow().len()
    }

    pub fn get_next_id(&self) -> u64 {
        (self.clients.borrow().len() + 1) as u64
    }

    pub fn add_client(&mut self, id: u64, client: Sender) -> bool {
        if self.clients.borrow().len() <= MAX_USERS_ALLOWED {
            self.clients.borrow_mut().insert(id, client);
            return true;
        }
        false
    }

    pub fn remove_client(&mut self, id: &u64) {
        self.clients.borrow_mut().remove(id);
    }
}

pub struct ServerHandler {
    pub id: u64,
    pub socket: Sender,
    pub manager: NetworkManagerRef
}

impl Handler for ServerHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Client connected");
        self.manager.borrow_mut().add_client(self.id, self.socket.clone());
        println!("Total clients: {}", self.manager.borrow().get_total_clients());
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        self.socket.send(msg)
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
        self.manager.borrow_mut().remove_client(&self.id);
        println!("Total clients: {}", self.manager.borrow().get_total_clients());
    }
}
