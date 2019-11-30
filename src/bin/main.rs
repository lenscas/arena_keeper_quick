use arena::{assets::to_load::load_all, states::StateManager};
use quicksilver::{
    geom::Vector,
    graphics::Color,
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Future, Result,
};
use std::{rc::Rc, sync::Mutex};

pub struct MainState<'a> {
    assets: Asset<StateManager<'a>>,
}
impl State for MainState<'static> {
    fn new() -> Result<Self> {
        Ok(Self {
            assets: Asset::new(load_all().and_then(|v| StateManager::new(v))),
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let test = Rc::new(Mutex::new(window));
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
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.assets.execute(|state| state.event(event, window))
    }
}

pub fn main() {
    run::<MainState>("Arena", Vector::new(800, 600), Settings::default());
}
