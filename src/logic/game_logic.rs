use engine::game::Game;

pub struct GameLogic {
    game: Game
}

impl GameLogic {
    pub fn new(_game: Game) -> GameLogic {
        GameLogic { game: _game }
    }

    pub fn init(&self) {
    }

    pub fn update(&self) {
        self.game.broadcast("SENT 60 times");
    }
}
