extern crate crossbeam;
extern crate ws;

mod logic;
mod engine;

use ws::listen;
use engine::network::NetworkManager;
use engine::client_handler::ClientHandler;
use engine::game_manager::GameManager;
use std::thread;
use crossbeam::channel as channel;

fn main() {
    let (gtx, grx): (channel::Sender<String>, channel::Receiver<String>) = channel::unbounded();
    let (ntx, nrx): (channel::Sender<String>, channel::Receiver<String>) = channel::unbounded();

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
                let handler = ClientHandler {
                    socket: client,
                    game_manager_bus: gtx.clone(),
                    network_manager_bus: ntx.clone()
                };
                handler
            }).unwrap();
        });
    });
}
