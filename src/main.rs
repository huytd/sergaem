extern crate crossbeam;
extern crate ws;

mod logic;
mod engine;

use ws::listen;
use engine::network::NetworkManager;
use engine::server_handler::ServerHandler;
use engine::game_manager::GameManager;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    let (gtx, grx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (ntx, nrx): (Sender<String>, Receiver<String>) = mpsc::channel();

    crossbeam::scope(|scope| {
        scope.spawn(|| {
            let mut game_manager = GameManager::new(grx);
            game_manager.listen();
            game_manager.create_game();
            game_manager.start_game(1u64);
        });
        scope.spawn(|| {
            let network_manager = NetworkManager::new(nrx);
            network_manager.listen();
        });
        let gtx = gtx.clone();
        let ntx = ntx.clone();
        scope.spawn(move || {
            listen("127.0.0.1:3123", |client| {
                let server = ServerHandler {
                    socket: client,
                    game_manager_bus: gtx.clone(),
                    network_manager_bus: ntx.clone()
                };
                server
            }).unwrap();
        });
    });
}
