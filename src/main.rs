#[macro_use]
extern crate lazy_static;
extern crate ws;

mod network;

use ws::listen;
use network::{ServerHandler, NetworkManager};

fn main() {
    let mut network_manager = NetworkManager::new_ref();

    listen("127.0.0.1:3123", |client| {
        let server = ServerHandler {
            id: network_manager.borrow().get_next_id(),
            socket: client,
            manager: network_manager.clone()
        };
        server
    }).unwrap();
}
