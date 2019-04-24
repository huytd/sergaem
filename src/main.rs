
extern crate crossbeam;
extern crate ws;

mod logic;
mod engine;

use ws::listen;
use engine::network::NetworkManager;
use engine::server_handler::ServerHandler;
use engine::game_manager::GameManager;

fn main() {
    crossbeam::scope(|scope| {
        scope.spawn(|| {
            let mut game_manager = GameManager::new();
            game_manager.create_game();
            game_manager.start_game(1u64);
        });
        scope.spawn(|| {
            let network_manager = NetworkManager::new();
        });
        scope.spawn(|| {
            listen("127.0.0.1:3123", |client| {
                let server = ServerHandler {
                    socket: client
                };
                server
            }).unwrap();
        });
    });
}
