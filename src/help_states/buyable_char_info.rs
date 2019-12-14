use crate::{
    assets::loaded::{Fonts, Images},
    mergui_wrapper::success_button,
    structs::{BuyableCharacter, FullContext},
};
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, FontStyle},
    prelude::{Image, Img, Transform},
};

use mergui::{
    channels::{BasicClickable, Clickable},
    LayerId, Response,
};

pub struct BuyableInfo {
    cost: (Rectangle, Image),
    buy_button: Response<BasicClickable>,
    image: Images,
    name: (Rectangle, Image),
    species: (Rectangle, Image),
    layer: LayerId,
}

impl BuyableInfo {
    pub fn new(chosen_character: &BuyableCharacter, context: &mut FullContext) -> BuyableInfo {
        let layer = context.simple_context.gui.add_layer();
        let image = chosen_character.get_image();
        let buy_button = context
            .simple_context
            .gui
            .add_widget(
                success_button(
                    context.simple_context.assets,
                    Rectangle::new((704, 503), (78, 38)),
                    "Buy",
                )
                .unwrap(),
                &layer,
            )
            .unwrap();
        let text = String::from("$") + &chosen_character.cost.to_string();
        let cost = (
            Rectangle::new((623, 503), (78, 38)),
            context
                .simple_context
                .assets
                .font(&Fonts::Font)
                .render(&text, &FontStyle::new(50.1, Color::BLACK))
                .unwrap(),
        );
        let name = (
            Rectangle::new((542, 15), (238, 34)),
            context
                .simple_context
                .assets
                .font(&Fonts::Font)
                .render(
                    &chosen_character.get_name(),
                    &FontStyle::new(50.1, Color::BLACK),
                )
                .unwrap(),
        );
        let species = (
            Rectangle::new((542, 61), (238, 34)),
            context
                .simple_context
                .assets
                .font(&Fonts::Font)
                .render(
                    &String::from(chosen_character.get_species()),
                    &FontStyle::new(50.1, Color::BLACK),
                )
                .unwrap(),
        );
        Self {
            cost,
            buy_button,
            image,
            name,
            species,
            layer,
        }
    }
    pub fn did_buy(&mut self, _context: &mut FullContext) -> bool {
        self.buy_button.channel.has_clicked()
    }
    pub fn set_state(&self, state: bool) {
        self.layer.set_is_active(state);
    }
    pub fn draw(&mut self, context: &mut FullContext) {
        let z = context.simple_context.get_z();
        context.simple_context.window.draw_ex(
            &self.cost.0,
            Img(&self.cost.1),
            Transform::IDENTITY,
            z,
        );
        let z = context.simple_context.get_z();
        context.simple_context.window.draw_ex(
            &self.name.0,
            Img(&self.name.1),
            Transform::IDENTITY,
            z,
        );
        let z = context.simple_context.get_z();
        context.simple_context.window.draw_ex(
            &self.species.0,
            Img(&self.species.1),
            Transform::IDENTITY,
            z,
        );
        /*
        context.simple_context.push_widget(self.buy_button.clone());
        context.simple_context.push_widget(self.cost.clone());
        context.simple_context.push_widget(self.name.clone());
        context.simple_context.push_widget(self.species.clone());
        */
        context.simple_context.window.draw(
            &Rectangle::new((403, 0), (130, 130)),
            Img(context.simple_context.assets.image(&self.image)),
        );
    }
}
