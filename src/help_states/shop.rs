use crate::{
    assets::loaded::{AssetManager, Fonts},
    help_states::characters::Characters,
    help_states::BuyableInfo,
    states::OpenWindow,
    structs::{gui_2::success_button, BuyableCharacter, FullContext, SimpleContext},
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
    money: u32,
    assets: Vec<(Response<BasicClickable>, BuyableCharacter)>,
    go_to_game_button: Response<BasicClickable>,
    selected: Option<(usize, BuyableInfo)>,
    show_money: Image,
    layer: LayerId,
}

impl Shop {
    pub fn new(context: &mut SimpleContext) -> Self {
        let layer = context.gui.add_layer();
        let assets = (0..3)
            .map(|_| BuyableCharacter::new(context.assets))
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
            .collect();
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
            money: money_amount,
            assets,
            go_to_game_button,
            selected: None,
            show_money,
            layer,
        }
    }
    fn get_show_money_image(assets: &AssetManager, money: u32) -> Image {
        Font::render(
            assets.font(&Fonts::Font),
            &money.to_string(),
            &FontStyle::new(50.1, Color::BLACK),
        )
        .unwrap()
    }
    pub fn update(&mut self, context: &mut FullContext, characters_state: &mut Characters) {
        if let Some(selected) = &mut self.selected {
            if selected.1.did_buy(context) {
                let (_, bought) = self.assets.remove(selected.0);
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
    pub fn set_state<'a>(&self, state: bool) {
        self.layer.set_is_active(state);
        if let Some(selected) = &self.selected {
            selected.1.set_state(state);
        }
    }
    pub fn render(&mut self, context: &mut FullContext) -> Result<()> {
        //context.push_widget(self.up_button.clone().unwrap());
        //context.simple_context.push_widget(self.show_money.clone());
        let z = context.simple_context.get_z();
        context.simple_context.window.draw_ex(
            &Rectangle::new((10, 10), (30, 20)),
            Img(&self.show_money),
            Transform::IDENTITY,
            z,
        );
        if let Some(info) = &mut self.selected {
            info.1.draw(context);
        }
        Ok(())
    }
}
