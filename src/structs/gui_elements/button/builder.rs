use quicksilver::graphics::FontStyle;
use quicksilver::Result;

use super::State;
use crate::structs::gui_elements::finalize::Finalize;

use quicksilver::graphics::Font;

pub struct Builder {
    pub normal: String,
    pub hovered: String,
    pub active: String,
}
impl Builder {
    pub fn new(normal: String, hovered: String, active: String) -> Self {
        Self {
            normal,
            hovered,
            active,
        }
    }
    pub fn new_single_text(text: String) -> Self {
        Self {
            normal: text.clone(),
            hovered: text.clone(),
            active: text,
        }
    }
}
impl Finalize for Builder {
    type to = State;
    fn to_state(self, font: Font, style: FontStyle) -> Result<(Font, Self::to)> {
        State::new(font, &style, &self.normal, &self.hovered, &self.active)
    }
}
