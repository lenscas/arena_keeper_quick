use crate::{
    assets::loaded::AssetManager,
    states::{GameState, Screen},
    structs::{
        gui_2::{button::State, ButtonBackground, Combined, Interaction},
        SimpleContext,
    },
};
use quicksilver::{geom::Rectangle, Result};

pub struct MainMenu {
    play_button: Combined<State, ButtonBackground>,
    settings_button: Combined<State, ButtonBackground>,
}
impl MainMenu {
    pub fn new(assets: &AssetManager) -> Self {
        let play_button = ButtonBackground::new_success(
            assets,
            Rectangle::new((200, 150), (400, 100)),
            "Play".to_string(),
        );
        let settings_button = ButtonBackground::new_success(
            assets,
            Rectangle::new((200, 270), (400, 100)),
            "Settings".to_string(),
        );

        Self {
            play_button,
            settings_button,
        }
    }
}
impl Screen for MainMenu {
    fn update(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        if context.get_interaction(&mut self.play_button) == Interaction::Clicked {
            Ok(Some(Box::new(GameState::new(rand::random(), context))))
        } else if context.get_interaction(&mut self.settings_button) == Interaction::Clicked {
            Ok(None)
        } else {
            Ok(None)
        }
    }
    fn draw(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        context.push_widget(self.play_button.clone());
        context.push_widget(self.settings_button.clone());
        Ok(None)
    }
}
