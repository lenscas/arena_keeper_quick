use crate::{
    assets::loaded::AssetManager,
    mergui_wrapper::success_button,
    states::{GameState, Screen},
    structs::SimpleContext,
};
use quicksilver::{geom::Rectangle, Result};

use mergui::{
    channels::{BasicClickable, Clickable},
    Context, LayerId, Response,
};

pub struct MainMenu {
    play_button: Response<BasicClickable>,
    settings_button: Response<BasicClickable>,
    _layer: LayerId,
}
impl MainMenu {
    pub fn new(assets: &AssetManager, gui: &mut Context) -> Self {
        let layer = gui.add_layer();
        let play_button =
            success_button(assets, Rectangle::new((200, 150), (400, 100)), "Play").unwrap();
        let settings_button =
            success_button(assets, Rectangle::new((200, 270), (400, 100)), "Settings").unwrap();
        let play_button = gui.add_widget(play_button, &layer).unwrap();
        let settings_button = gui.add_widget(settings_button, &layer).unwrap();
        Self {
            _layer: layer,
            play_button,
            settings_button,
        }
    }
}
impl Screen for MainMenu {
    fn update(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        if self.play_button.channel.has_clicked() {
            Ok(Some(Box::new(GameState::new(rand::random(), context))))
        } else if self.settings_button.channel.has_clicked() {
            println!("not implemented");
            Ok(None)
        } else {
            Ok(None)
        }
    }
    fn draw(&mut self, _: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }
}
