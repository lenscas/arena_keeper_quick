use crate::{
    assets::loaded::AssetManager,
    states::MainMenu,
    structs::{gui_2::Context, SimpleContext},
};
use quicksilver::{graphics::Color, lifecycle::Window, Result};

pub struct StateManager {
    assets: AssetManager,
    current_screen: Box<dyn Screen>,
}
impl StateManager {
    fn set_current_screen(&mut self, screen: Option<Box<dyn Screen>>) {
        if let Some(screen) = screen {
            self.current_screen = screen;
        }
    }
    pub fn new(assets: AssetManager) -> Result<Self> {
        let current_screen = Box::new(MainMenu::new(&assets)) as Box<dyn Screen>;
        Ok(Self {
            assets,
            current_screen,
        })
    }
    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let screen = {
            let mut context = SimpleContext::new(window, Context::new(), &self.assets);
            let screen = self.current_screen.draw(&mut context)?;
            context.render_gui();
            screen
        };
        self.set_current_screen(screen);
        Ok(())
    }
    pub fn event(&mut self, window: &mut Window) -> Result<()> {
        let screen = {
            let mut context = SimpleContext::new(window, Context::new(), &self.assets);
            self.current_screen.event(&mut context)?
        };
        self.set_current_screen(screen);
        Ok(())
    }
    pub fn update(&mut self, window: &mut Window) -> Result<()> {
        let screen = {
            let mut context = SimpleContext::new(window, Context::new(), &self.assets);
            self.current_screen.update(&mut context)?
        };
        self.set_current_screen(screen);
        Ok(())
    }
}

pub trait Screen {
    fn update(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>>;
    fn draw(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>>;
    fn event(&mut self, _: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }
}
