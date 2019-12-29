use crate::assets::loaded::{AssetManager, Fonts};
use mergui::core::Text;
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, FontStyle},
    Result,
};
pub fn text(assets: &AssetManager, location: Rectangle, text: &str) -> Result<Text> {
    let font = assets.font(&Fonts::Font);
    let normal = FontStyle::new(50.0, Color::BLACK);
    let text = font.render(&text, &normal)?;
    Ok(Text { location, text })
}
