use crate::{
    assets::loaded::AssetManager,
    states::MainMenu,
    structs::{gui_2::Context, SimpleContext},
};
use mergui::Context as GContext;
use quicksilver::{
    graphics::Color,
    lifecycle::{Event, Window},
    Result,
};

pub struct StateManager<'a> {
    assets: AssetManager,
    current_screen: Box<dyn Screen>,
    context: GContext<'a>,
}
impl<'a> StateManager<'a> {
    fn set_current_screen(&mut self, screen: Option<Box<dyn Screen>>) {
        if let Some(screen) = screen {
            self.current_screen = screen;
        }
    }
    pub fn new(assets: AssetManager) -> Result<Self> {
        let current_screen = Box::new(MainMenu::new(&assets)) as Box<dyn Screen>;
        let mut context = GContext::new((0, 0).into(), 1000);
        let layer_id = context.add_layer();
        let _widget_id = context.add_widget(
            mergui::widgets::Image {
                image: "test_button".to_string(),
                location: quicksilver::prelude::Rectangle::new((100, 100), (200, 100)),
            },
            layer_id,
        );
        Ok(Self {
            assets,
            current_screen,
            context,
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
        self.context.render(&self.assets, window);
        Ok(())
    }
    pub fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        let screen = {
            let mut context = SimpleContext::new(window, Context::new(), &self.assets);
            self.current_screen.event(event, &mut context)?
        };
        self.set_current_screen(screen);
        self.context.event(event, window);
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
    fn event(&mut self, _event: &Event, _: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }
}
