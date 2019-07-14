use immi::UiState;
use quicksilver::graphics::ImmiRender;
use immi::DrawContext;
use quicksilver::graphics::Font;
use quicksilver::graphics::Image;
use quicksilver::graphics::FontStyle;
use quicksilver::Result;
use immi::{
    Alignment,
    widgets::{Interaction, image_button}
};

pub struct State {
    pub normal: Image,
    pub hovered: Image,
    pub active: Image,
}

impl State {
    pub fn new_single_raw(image : Image) -> Result<Self> {
        let normal = image.clone();
        let hovered = image.clone();
        let active = image;
        Ok( Self { normal, hovered, active })
    }
    pub fn new(font: Font, style: &FontStyle,normal : &str, hover : &str, active : &str) -> Result<(Font, Self)> {
        let normal = font.render(normal, &style)?;
        let hovered = font.render(hover, &style)?;
        let active = font.render(active, &style)?;
        Ok((font, Self { normal, hovered, active }))
    }
    pub fn new_single_text(font: Font, style: &FontStyle,text : &str) -> Result<(Font,Self)> {
        let normal = font.render(text, &style)?;
        let hovered = font.render(text, &style)?;
        let active = font.render(text, &style)?;
        Ok((font, Self { normal, hovered, active }))
    }
    pub fn render(&self,draw : &DrawContext<ImmiRender>,state : &mut UiState, allign : Alignment) -> Interaction {
        image_button::draw(draw, state, &self.normal, &self.hovered, &self.active, &allign)
    }
}