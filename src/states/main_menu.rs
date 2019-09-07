use crate::states::GameState;
use crate::states::Screen;
use crate::structs::SimpleContext;

use quicksilver::Result;

#[derive(Default)]
pub struct MainMenu {}
impl MainMenu {
    pub fn new() -> Self {
        Self {}
    }
}
impl Screen for MainMenu {
    fn update(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        Ok(Some(Box::new(GameState::new(rand::random(), context))))
    }
    fn draw(&mut self, _: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }
}
