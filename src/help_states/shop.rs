use crate::help_states::BuyableInfo;
use crate::generated::assets::loaded::AssetManager;
use crate::generated::assets::loaded::Fonts;
use crate::structs::gui_2::button::Builder;
use crate::structs::gui_2::button::State;
use crate::structs::gui_2::Interaction;
use crate::structs::FullContext;
use crate::{help_states::characters::Characters, structs::BuyableCharacter};
use quicksilver::geom::Rectangle;
use quicksilver::{
    graphics::{Color, FontStyle},
    Result,
};
use crate::states::game_state::OpenWindow;

#[derive(Default)]
pub struct Shop {
    money: u32,
    assets: Vec<(State, BuyableCharacter)>,
    go_to_game_button : Option<State>,
    selected : Option<(usize, BuyableInfo)>
}

impl Shop {
    pub fn new() -> Self {
        Self {
            money: 100,
            assets: Vec::new(),
            go_to_game_button : None,
            selected : None
        }
    }
    pub fn first_render(&mut self, assets: &AssetManager) {
        let chars = vec![
            BuyableCharacter::new(),
            BuyableCharacter::new(),
            BuyableCharacter::new(),
        ];
        self.assets = chars
            .iter()
            .cloned()
            .enumerate()
            .map(|(count, v)| {
                let name = v.get_name();
                let builder = Builder::new_single_text(name);
                let style = FontStyle::new(50.1, Color::BLACK);
                let button = builder
                    .to_state(
                        Fonts::Font,
                        style,
                        assets,
                        Rectangle::new((10, 10 + (count as i32 * 50)), (90, 50)),
                    )
                    .unwrap();
                (button, v)
            })
            .collect();
        let builder = Builder::new_single_text("World".to_string());
        let style = FontStyle::new(50.1, Color::BLACK);
        let button = builder
            .to_state(
                Fonts::Font,
                style,
                assets,
                Rectangle::new((10,555), (40, 30)),
            )
            .unwrap();
        self.go_to_game_button = Some(button);
    }
    pub fn update(&mut self, context : &mut FullContext,characters_state: &mut Characters,) {
        if let Some(selected) = &mut self.selected {
            if selected.1.did_buy(context) {
                let bought = self.assets.remove(selected.0).1;
                if bought.cost < self.money {
                    characters_state.add_character(bought);
                    self.selected = None;
                }
            }
        }
        let selected = self.assets
            .iter_mut()
            .enumerate()
            .map(|(count, button)| (count,context.get_interaction(&mut button.0)))
            .filter(|v| v.1 == Interaction::Clicked)
            .collect::<Vec<_>>()
            .iter()
            .find(|v| v.1 == Interaction::Clicked)
            .map(|v| (v.0, BuyableInfo::new(&self.assets[v.0].1, context)));
        if let Some(selected) = selected {
            self.selected = Some(selected);
        }
        if self.go_to_game_button.iter_mut().map(|v| context.get_interaction(v)).any(|v| v == Interaction::Clicked) {
            context.set_next_screen(Some(OpenWindow::Game));
        }
    }
    pub fn render(
        &mut self,
        context: &mut FullContext,
    ) -> Result<()> {
        self.assets
            .iter()
            .cloned()
            .for_each(|(button,_)| {
                context.push_widget(button);
            });
        context.push_widget(self.go_to_game_button.clone().unwrap());
        if let Some(info) = &mut self.selected {
            info.1.draw(context);
        }
        Ok(())
    }
}
