extern crate arena;
use arena::states::game_state::GameState;

use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings, State, Window},
    Result,
};

pub struct MainState {
    game_state: GameState,
}
impl State for MainState {
    fn new() -> Result<Self> {
        Ok(Self {
            game_state: GameState::new()?,
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.game_state.draw(window)
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.game_state.update(window)
    }
}

pub fn main() {
    run::<MainState>("Arena", Vector::new(800, 600), Settings::default());
}
