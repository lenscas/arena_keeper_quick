use crate::{
    generated::assets::loaded::{Fonts, Images},
    structs::{
        gui_2::{button::State, ButtonBackground, Combined, Image, Interaction},
        BuyableCharacter, FullContext,
    },
};
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, FontStyle},
};

pub struct BuyableInfo {
    cost: Image,
    buy_button: Combined<State, ButtonBackground>,
    image: Images,
    name: Image,
    species: Image,
}

impl BuyableInfo {
    pub fn new(chosen_character: &BuyableCharacter, context: &mut FullContext) -> BuyableInfo {
        let assets = context.get_assets();
        let image = chosen_character.get_image();
        let buy_button = ButtonBackground::new_success(
            assets,
            Rectangle::new((704, 503), (78, 38)),
            "Buy".to_string(),
        );
        let text = String::from("$") + &chosen_character.cost.to_string();
        let cost = Image::new(
            assets
                .font(&Fonts::Font)
                .render(&text, &FontStyle::new(50.1, Color::BLACK))
                .unwrap(),
            Rectangle::new((623, 503), (78, 38)),
        );
        let name = Image::new(
            assets
                .font(&Fonts::Font)
                .render(
                    &chosen_character.get_name(),
                    &FontStyle::new(50.1, Color::BLACK),
                )
                .unwrap(),
            Rectangle::new((542, 15), (238, 34)),
        );
        let species = Image::new(
            assets
                .font(&Fonts::Font)
                .render(
                    &String::from(chosen_character.get_species()),
                    &FontStyle::new(50.1, Color::BLACK),
                )
                .unwrap(),
            Rectangle::new((542, 61), (238, 34)),
        );
        Self {
            cost,
            buy_button,
            image,
            name,
            species,
        }
    }
    pub fn did_buy(&mut self, context: &mut FullContext) -> bool {
        context.get_interaction(&mut self.buy_button) == Interaction::Clicked
    }
    pub fn draw(&mut self, context: &mut FullContext) {
        context.push_widget(self.buy_button.clone());
        context.push_widget(self.cost.clone());
        context.push_widget(self.name.clone());
        context.push_widget(self.species.clone());
        context.draw_image(&Rectangle::new((403, 0), (130, 130)), self.image);
    }
}
//full info?
/*
Point: <410, 152>
size: <366, 332>
*/
//buy button
/*
Point: <704, 503>
size: <78, 38>
image ?
Point: <403, 13>
size: <132, 124>
name
Point: <542, 15>
size: <238, 34>

*/

/*

inside first click

inside first click
Point: <547, 61>
size: <227, 30>
inside first click

inside first click
Point: <404, 507>
size: <175, 34>
inside first click

inside first click
Point: <23, 15>
size: <322, 29>
inside first click
Point: <21, 512>
size: <325, 26>
inside first click
Point: <26, 55>
size: <317, 453>
*/
