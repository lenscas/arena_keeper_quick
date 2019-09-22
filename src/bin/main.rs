extern crate arena;
use arena::generated::assets::to_load::load_all;
use arena::modules::{
    handle_files::{get_all_mod_paths, load_mod_info},
    structs::Module,
};
use arena::states::StateManager;
use quicksilver::combinators::join_all;
use quicksilver::lifecycle::Asset;
use quicksilver::Future;
use std::rc::Rc;
use std::sync::Mutex;

use quicksilver::{
    geom::Vector,
    graphics::Color,
    lifecycle::{run, Settings, State, Window},
    Result,
};

pub struct MainState {
    assets: Asset<StateManager>,
    test: Asset<Vec<Module>>,
}
impl State for MainState {
    fn new() -> Result<Self> {
        let b = get_all_mod_paths()
            .and_then(|v| join_all(v.iter().map(|x| load_mod_info(x)).collect::<Vec<_>>()));
        Ok(Self {
            assets: Asset::new(load_all().and_then(|v| StateManager::new(v))),
            test: Asset::new(b),
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let test = Rc::new(Mutex::new(window));
        self.test.execute(|_| Ok(()))?;
        self.assets.execute_or(
            |state| {
                let mut b = test.lock().unwrap();
                state.draw(&mut b)
            },
            || {
                let mut b = test.lock().unwrap();

                b.clear(Color::RED)
            },
        )
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.assets.execute(|state| state.update(window))
    }
}

pub fn main() {
    run::<MainState>("Arena", Vector::new(800, 600), Settings::default());
}
