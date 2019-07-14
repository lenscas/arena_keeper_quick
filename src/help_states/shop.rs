use crate::structs::gui_elements::load_from;
use crate::structs::gui_elements::button::State;
use crate::structs::gui_elements::button::Builder;
use crate::structs::gui_elements::Render;
use crate::{
    help_states::characters::Characters,
    structs::BuyableCharacter,
};

use quicksilver::{
    Result,
    graphics::{
        Color
    },
    lifecycle::{
        Window
    }
};

use immi::{
    Alignment,
    widgets::{Interaction}
};

pub struct Shop {
    money : u32,
    ui_state: immi::UiState,
    assets: Vec<(Render<State>,BuyableCharacter)>
}

impl Default for Shop {
    fn default() -> Self {
        Self::new()
    }
}

impl Shop {
    pub fn new() -> Self {
        let chars = vec![
            BuyableCharacter::new(),
            BuyableCharacter::new(),
            BuyableCharacter::new(),
        ];
        let chars_ui = chars.iter().cloned().enumerate().map(
            |(count,v)| {
                let name = v.get_name();
                let builder = Builder::new_single_text(name);
                let asset = load_from(
                    "font.ttf".into(),
                    50.0,
                    Color::BLACK,
                    builder
                );
                let render = Render::new(
                    Box::new(
                        move |r ,draw | {
                            let mut place = count as f32 * 0.22;
                            //*
                            let alignment = if place < 0.75 {
                                Alignment::top_left()
                            } else if place < 1.3 {
                                place -= 0.75;
                                Alignment::left()
                            } else if place < 2.5 {
                                place -= 1.5;
                                Alignment::bottom_left()
                            } else {
                                return;
                            };
                            //*/
                            r(draw.rescale(0.4,0.4,&alignment).uniform_margin(place,0.0,0.0,0.0))
                        }
                    ),
                    asset
                );
                (render,v)
            }
        ).collect();
        Self {
            money : 100,
            ui_state: Default::default(),
            assets: chars_ui
        }
    }
    pub fn render(&mut self, window : &mut Window, characters_state : &mut Characters) -> Result<()> {
        let ui_state = &mut self.ui_state;
        let mut to_remove : Vec<usize> = Vec::new();


        for (count,v) in self.assets.iter_mut().enumerate() {
            v.0.render(window,ui_state,|draw,state,button| {
                let interaction = button.render(&draw,state,Alignment::top_left());
                if Interaction::Clicked == interaction {
                    to_remove.push(count);
                };
            })?;
        };
        to_remove.iter().for_each(|v|{
            let item = self.assets.remove(*v);
            if item.1.cost < self.money {
                self.money -= item.1.cost;
                characters_state.add_character(item.1);
            }
            println!("{}",self.money);

        });

        Ok(())
    }
}