use crate::structs::SimpleContext;
use crate::{
    generated::assets::loaded::{AssetManager, Fonts},
    help_states::characters::Characters,
    help_states::BuyableInfo,
    states::OpenWindow,
    structs::BuyableCharacter,
    structs::{
        gui_2::{
            button::{Builder, State},
            ButtonBackground, Combined, Image, Interaction,
        },
        FullContext,
    },
};
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, Font, FontStyle},
    Result,
};

pub struct Shop {
    money: u32,
    assets: Vec<(State, BuyableCharacter)>,
    go_to_game_button: Combined<State, ButtonBackground>,
    selected: Option<(usize, BuyableInfo)>,
    show_money: Image,
    _up_button: Combined<State, ButtonBackground>,
}

impl Shop {
    pub fn new(context: &mut SimpleContext) -> Self {
        let assets = (0..3)
            .map(|_| BuyableCharacter::new(context.assets))
            .enumerate()
            .map(|(count, v)| {
                let name = v.get_name();
                let builder = Builder::new_single_text(name);
                let style = FontStyle::new(50.1, Color::BLACK);
                let button = builder
                    .to_state(
                        Fonts::Font,
                        style,
                        context.assets,
                        Rectangle::new((10, 90 + (count as i32 * 50)), (90, 50)),
                    )
                    .unwrap();
                (button, v)
            })
            .collect();
        let up_button = ButtonBackground::new_success(
            context.assets,
            Rectangle::new((10, 40), (90, 50)),
            "Up".to_string(),
        );
        let go_to_game_button = ButtonBackground::new_success(
            context.assets,
            Rectangle::new((10, 555), (55, 40)),
            "World".to_string(),
        );
        let money_amount = 100;
        let show_money = Self::get_show_money_image(context.assets, money_amount);
        Self {
            money: money_amount,
            assets,
            go_to_game_button,
            selected: None,
            show_money,
            _up_button: up_button,
        }
    }
    fn get_show_money_image(assets: &AssetManager, money: u32) -> Image {
        Image::new(
            Font::render(
                assets.font(&Fonts::Font),
                &money.to_string(),
                &FontStyle::new(50.1, Color::BLACK),
            )
            .unwrap(),
            Rectangle::new((10, 10), (30, 20)),
        )
    }
    pub fn update(&mut self, context: &mut FullContext, characters_state: &mut Characters) {
        if let Some(selected) = &mut self.selected {
            if selected.1.did_buy(context) {
                let bought = self.assets.remove(selected.0).1;
                if bought.cost < self.money {
                    self.money -= bought.cost;
                    self.show_money =
                        Self::get_show_money_image(context.simple_context.get_assets(), self.money);
                    characters_state.add_character(bought);
                    self.selected = None;
                }
            }
        }
        let selected = self
            .assets
            .iter_mut()
            .enumerate()
            .map(|(count, button)| (count, context.simple_context.get_interaction(&mut button.0)))
            .filter(|v| v.1 == Interaction::Clicked)
            .collect::<Vec<_>>()
            .iter()
            .find(|v| v.1 == Interaction::Clicked)
            .map(|v| (v.0, BuyableInfo::new(&self.assets[v.0].1, context)));
        if let Some(selected) = selected {
            self.selected = Some(selected);
        }
        if context
            .simple_context
            .get_interaction(&mut self.go_to_game_button)
            == Interaction::Clicked
        {
            context.set_next_screen(Some(OpenWindow::Game));
        }
    }
    pub fn render(&mut self, context: &mut FullContext) -> Result<()> {
        self.assets.iter().cloned().for_each(|(button, _)| {
            context.simple_context.push_widget(button);
        });
        //context.push_widget(self.up_button.clone().unwrap());
        context.simple_context.push_widget(self.show_money.clone());
        context
            .simple_context
            .push_widget(self.go_to_game_button.clone());
        if let Some(info) = &mut self.selected {
            info.1.draw(context);
        }
        Ok(())
    }
}
