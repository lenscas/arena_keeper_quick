use crate::generated::assets::loaded::AssetManager;
use quicksilver::geom::Rectangle;
use crate::structs::FullContext;
use crate::generated::assets::loaded::Fonts;
use crate::structs::gui_2::Interaction;
use crate::structs::gui_2::button::Builder;
use crate::structs::gui_2::button::State;
use crate::{help_states::characters::Characters, structs::BuyableCharacter};
use quicksilver::{graphics::{FontStyle,Color}, Result};


#[derive(Default)]
pub struct Shop {
    money: u32,
    assets: Vec<(State, BuyableCharacter)>,
}

impl Shop {
    pub fn new() -> Self {
        Self {
            money: 100,
            assets: Vec::new()
        }
    }
    pub fn first_render(&mut self, assets :&AssetManager) {
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
                let button =builder.to_state(Fonts::Font, style, assets, Rectangle::new((10,10 + (count as i32 * 50) ), (90,50))).unwrap();
                (button,v)
            })
            .collect();
        
    }
    pub fn render(&mut self, context : &mut FullContext, characters_state: &mut Characters) -> Result<()> {
        
        let mut to_remove: Vec<usize> = Vec::new();
        self.assets.iter().cloned().enumerate().for_each(|(count, button )| {
            let interaction = context.push_widget(button.0);
            if let Interaction::Clicked = interaction {to_remove.push(count)}
        });
        to_remove.iter().for_each(|v| {
            let item = self.assets.remove(*v);
            if item.1.cost < self.money {
                self.money -= item.1.cost;
                characters_state.add_character(item.1);
            }
            println!("{}", self.money);
        });
        Ok(())
    }
}
