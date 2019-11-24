use crate::{
    assets::loaded::AssetManager,
    states::{GameState, Screen},
    structs::{gui_2::success_button, SimpleContext},
};
use quicksilver::{geom::Rectangle, Result};

use mergui::{
    channels::{BasicClickable, Clickable},
    Context, Response,
};

pub struct MainMenu {
    play_button: Response<BasicClickable>,
    settings_button: Response<BasicClickable>,
    layer: u64,
}
impl MainMenu {
    pub fn new(assets: &AssetManager, gui: &mut Context) -> Self {
        let layer = gui.add_layer();
        let play_button =
            success_button(assets, Rectangle::new((200, 150), (400, 100)), "Play").unwrap();
        let settings_button =
            success_button(assets, Rectangle::new((200, 270), (400, 100)), "Settings").unwrap();
        let play_button = gui.add_widget(play_button, layer).unwrap();
        let settings_button = gui.add_widget(settings_button, layer).unwrap();
        Self {
            layer,
            play_button,
            settings_button,
        }
    }
}
impl Screen for MainMenu {
    fn update(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        if self.play_button.channel.has_clicked() {
            context.gui.remove_layer(self.layer);
            Ok(Some(Box::new(GameState::new(rand::random(), context))))
        } else if self.settings_button.channel.has_clicked() {
            println!("not implemented");
            context.gui.remove_layer(self.layer);
            Ok(None)
        } else {
            Ok(None)
        }
    }
    fn draw(&mut self, _: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }
}
