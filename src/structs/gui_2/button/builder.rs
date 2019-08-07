use super::State;
use crate::generated::assets::loaded::AssetManager;
use crate::generated::assets::loaded::Fonts;
use quicksilver::geom::Rectangle;
use quicksilver::graphics::FontStyle;
use quicksilver::Result;

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
impl Builder {
    pub fn to_state(
        &self,
        font: Fonts,
        style: FontStyle,
        assets: &AssetManager,
        location: Rectangle,
    ) -> Result<State> {
        let font = assets.font(&font);
        Ok(State {
            normal: font.render(&self.normal, &style)?,
            hovered: font.render(&self.hovered, &style)?,
            active: font.render(&self.active, &style)?,
            location,
        })
    }
}
