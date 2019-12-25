use crate::{
    assets::loaded::{AssetManager, Fonts},
    help_states::BuyableInfo,
    mergui_wrapper::success_button,
    states::OpenWindow,
    structs::{BuyableCharacter, FullContext, SimpleContext},
};
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, Font, FontStyle, Image},
    prelude::{Img, Transform},
    Result,
};

use mergui::{
    channels::{BasicClickable, Clickable},
    core::TextButtonConfig,
    LayerId, Response,
};

pub struct Shop {
    assets: Vec<(Response<BasicClickable>, BuyableCharacter)>,
    go_to_game_button: Response<BasicClickable>,
    selected: Option<(usize, BuyableInfo)>,
    show_money: (u32, Image),
    layer: LayerId,
}

impl Shop {
    pub fn new(context: &mut SimpleContext, buyable_chars: &[BuyableCharacter]) -> Self {
        let layer = context.gui.add_layer();
        let assets = Self::make_buttons_for_buyable(buyable_chars, context, &layer);

        let go_to_game_button = context
            .gui
            .add_widget(
                success_button(context.assets, Rectangle::new((10, 555), (55, 40)), "World")
                    .unwrap(),
                &layer,
            )
            .unwrap();
        let money_amount = 100;
        let show_money = Self::get_show_money_image(context.assets, money_amount);
        Self {
            assets,
            go_to_game_button,
            selected: None,
            show_money: (money_amount, show_money),
            layer,
        }
    }
    fn make_buttons_for_buyable(
        characters: &[BuyableCharacter],
        context: &mut SimpleContext,
        layer: &LayerId,
    ) -> Vec<(Response<BasicClickable>, BuyableCharacter)> {
        characters
            .iter()
            .cloned()
            .enumerate()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(count, v)| {
                let name = v.get_name();
                let style = FontStyle::new(50.1, Color::BLACK);

                let button = TextButtonConfig {
                    text: context
                        .assets
                        .font(&Fonts::Font)
                        .render(&name, &style)
                        .unwrap(),
                    location: Rectangle::new((10, 90 + (count as i32 * 50)), (90, 50)),
                };
                let button = context.gui.add_widget(button, &layer).unwrap();
                (button, v)
            })
            .collect()
    }
    fn get_show_money_image(assets: &AssetManager, money: u32) -> Image {
        Font::render(
            assets.font(&Fonts::Font),
            &money.to_string(),
            &FontStyle::new(50.1, Color::BLACK),
        )
        .unwrap()
    }
    pub fn reset_if_changed(&mut self, context: &mut FullContext) {
        if self.show_money.0 != context.state.money {
            self.show_money.0 = context.state.money;
            self.show_money.1 =
                Self::get_show_money_image(context.simple_context.assets, self.show_money.0);
        }
        let count = context.state.buyable_charachters.len() != self.assets.len();
        if count
            || context
                .state
                .buyable_charachters
                .iter()
                .enumerate()
                .any(|(key, character)| {
                    let owned = &self.assets[key];
                    owned.1 != *character
                })
        {
            self.assets = Self::make_buttons_for_buyable(
                &context.state.buyable_charachters,
                context.simple_context,
                &self.layer,
            );
            self.selected = None;
        }
    }
    pub fn update(&mut self, context: &mut FullContext) {
        self.reset_if_changed(context);
        if let Some(selected) = &mut self.selected {
            if selected.1.did_buy(context) {
                let bought = &context.state.buyable_charachters[selected.0];
                if bought.cost < self.show_money.0 {
                    let bought = context.state.buyable_charachters.remove(selected.0);
                    context.state.money -= bought.cost;
                    context.state.bought_characters.add_character(bought);
                    self.selected = None;
                }
            }
        }
        let selected = self
            .assets
            .iter_mut()
            .enumerate()
            .map(|(count, button)| (count, button.0.channel.has_clicked()))
            .find(|(_, has_clicked)| *has_clicked)
            .map(|v| (v.0, BuyableInfo::new(&self.assets[v.0].1, context)));
        if let Some(selected) = selected {
            self.selected = Some(selected);
        }
        if self.go_to_game_button.channel.has_clicked() {
            self.layer.set_is_active(false);
            context.set_next_screen(Some(OpenWindow::Game));
        }
    }
    pub fn set_state(&self, state: bool) {
        self.layer.set_is_active(state);
        if let Some(selected) = &self.selected {
            selected.1.set_state(state);
        }
    }
    pub fn render(&mut self, context: &mut FullContext) -> Result<()> {
        self.reset_if_changed(context);
        //context.push_widget(self.up_button.clone().unwrap());
        //context.simple_context.push_widget(self.show_money.clone());
        let z = context.simple_context.get_z();
        context.simple_context.window.draw_ex(
            &Rectangle::new((10, 10), (30, 20)),
            Img(&self.show_money.1),
            Transform::IDENTITY,
            z,
        );
        if let Some(info) = &mut self.selected {
            info.1.draw(context);
        }
        Ok(())
    }
}
