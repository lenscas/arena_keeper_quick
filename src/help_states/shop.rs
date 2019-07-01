use crate::{
    help_states::characters::Characters,
    structs::BuyableCharacter
};

use quicksilver::{
    Result,
    Future,
    combinators::result,
    graphics::{
        FontStyle,
        Color,
        ImmiStatus,
        create_immi_ctx,
        ImmiRender,
        Font,
        Image
    },
    lifecycle::{
        Asset,
        Window
    }
};

use immi::{
    Alignment,
    widgets::{Interaction, image_button}
};
pub struct Shop {
    money : u32,
    ui_state: immi::UiState,
    assets: Vec<(Asset<(Font, ButtonState)>, BuyableCharacter)>,
}
struct ButtonState {
    normal: Image,
    hovered: Image,
    active: Image,
}

impl ButtonState {
    fn new(font: Font, style: &FontStyle,name : String) -> Result<(Font, ButtonState)> {
        let normal = font.render(&name, &style)?;
        let hovered = font.render("Hovered Button", &style)?;
        let active = font.render("Active Button", &style)?;
        Ok((font, ButtonState { normal, hovered, active }))
    }
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
                BuyableCharacter::new()
            ];
        let chars_ui = chars.iter().cloned().map(
            |v| {
                let name = v.get_name();
                let asset = Asset::new(
                    Font::load("font.ttf").and_then(
                        move |font| {
                            let style = FontStyle::new(48.0, Color::BLACK);
                            result(ButtonState::new(font, &style, name))
                        }
                    )
                );
                (
                    asset,
                    v
                )
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
            v.0.execute(|(font, button)| {
                // Set up IMMI resources
                let ui_status = ImmiStatus::new(window);
                let mut ui_render = ImmiRender::new_with_window(window, font);
                let ui_context = create_immi_ctx(ui_status, &mut ui_render)
                    // Only take up half the screen with the immi widgets
                    .rescale(0.5, 0.5, &Alignment::top())
                    .margin( (count * 3) as f32 /10.0,0.0,0.0,0.0);

                // Draw a button widget and if it's clicked, print test
                let interaction = image_button::draw(&ui_context, ui_state, &button.normal, &button.hovered, &button.active, &Alignment::top());
                if Interaction::Clicked == interaction {
                    to_remove.push(count);
                };
                Ok(())
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