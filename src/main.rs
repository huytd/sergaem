extern crate ws;

mod logic;
mod engine;

use ws::listen;
use engine::network::{ServerHandler, NetworkManager};
use engine::game_manager::GameManager;

fn main() {
    let network_manager = NetworkManager::new_ref();
    let game_manager = GameManager::new_ref();
    game_manager.borrow_mut().create_game();
    game_manager.borrow().start_game(1u64);

    listen("127.0.0.1:3123", |client| {
        let server = ServerHandler {
            socket: client,
            manager: network_manager.clone(),
            game_manager: game_manager.clone()
        };
        server
    }).unwrap();
}
