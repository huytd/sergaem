extern crate ws;

mod network;
mod game;
mod game_manager;

use ws::listen;
use network::{ServerHandler, NetworkManager};
use game_manager::GameManager;

fn main() {
    let network_manager = NetworkManager::new_ref();
    let game_manager = GameManager::new_ref();
    game_manager.borrow_mut().create_game();

    listen("127.0.0.1:3123", |client| {
        let server = ServerHandler {
            id: network_manager.borrow().get_next_id(),
            socket: client,
            manager: network_manager.clone(),
            game_manager: game_manager.clone()
        };
        server
    }).unwrap();
}
