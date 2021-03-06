use crate::{
    assets::loaded::AssetManager,
    mergui_wrapper::success_button,
    states::{GameState, OptionsScreen, Screen},
    structs::SimpleContext,
};
use quicksilver::{geom::Rectangle, Result};

use mergui::{
    channels::{BasicClickable, Clickable},
    Context, LayerId, Response,
};

pub struct MainMenu {
    new_world_button: Response<BasicClickable>,
    open_world_button: Response<BasicClickable>,
    settings_button: Response<BasicClickable>,
    _layer: LayerId,
}
impl MainMenu {
    pub fn new(assets: &AssetManager, gui: &mut Context) -> Self {
        let layer = gui.add_layer();
        let new_world_button =
            success_button(assets, Rectangle::new((200, 120), (400, 100)), "New world").unwrap();
        let open_world_button =
            success_button(assets, Rectangle::new((200, 240), (400, 100)), "Load world").unwrap();

        let settings_button =
            success_button(assets, Rectangle::new((200, 360), (400, 100)), "Settings").unwrap();
        let new_world_button = gui.add_widget(new_world_button, &layer).unwrap();
        let open_world_button = gui.add_widget(open_world_button, &layer).unwrap();
        let settings_button = gui.add_widget(settings_button, &layer).unwrap();
        Self {
            _layer: layer,
            new_world_button,
            open_world_button,
            settings_button,
        }
    }
}
impl Screen for MainMenu {
    fn update(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        if self.new_world_button.channel.has_clicked() {
            Ok(Some(Box::new(GameState::new(rand::random(), context))))
        } else if self.open_world_button.channel.has_clicked() {
            let state = quicksilver::saving::load("arena_keeper", "1")?;
            Ok(Some(Box::new(GameState::from_saved(state, context))))
        } else if self.settings_button.channel.has_clicked() {
            Ok(Some(Box::new(OptionsScreen::new(context)?)))
        } else {
            Ok(None)
        }
    }
    fn draw(&mut self, _: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }
}
