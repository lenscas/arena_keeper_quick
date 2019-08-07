extern crate arena;
use arena::generated::assets::loaded::AssetManager;
use arena::generated::assets::to_load::load_all;
use arena::states::game_state::GameState;
use quicksilver::lifecycle::Asset;
use std::rc::Rc;
use std::sync::Mutex;

use quicksilver::{
    geom::Vector,
    graphics::Color,
    lifecycle::{run, Settings, State, Window},
    Result,
};

pub struct MainState {
    game_state: GameState,
    assets: Asset<AssetManager>,
}
impl State for MainState {
    fn new() -> Result<Self> {
        Ok(Self {
            game_state: GameState::new(rand::random()),
            assets: Asset::new(load_all()),
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let gamestate = &mut self.game_state;
        let test = Rc::new(Mutex::new(window));
        self.assets.execute_or(
            |asset| {
                let mut b = test.lock().unwrap();
                gamestate.draw(&mut b, asset)
            },
            || {
                let mut b = test.lock().unwrap();
                b.clear(Color::RED)
            },
        )
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        let gamestate = &mut self.game_state;
        self.assets.execute(|_| gamestate.update(window))
    }
}

pub fn main() {
    run::<MainState>("Arena", Vector::new(800, 600), Settings::default());
}
