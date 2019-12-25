use crate::assets::loaded::{AssetManager, Fonts};
use mergui::widgets::ButtonConfig;
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, FontStyle},
    Result,
};

pub fn success_button(
    assets: &AssetManager,
    location: Rectangle,
    text: &str,
) -> Result<ButtonConfig> {
    let skip_x = location.size.x / 100.0 * 15.0;
    let size_x = location.size.x - (skip_x * 2.0);
    let skip_y = location.size.y / 100.0 * 30.0;
    let size_y = location.size.y - (skip_y * 2.0);
    let button_location = Rectangle::new((skip_x, skip_y), (size_x, (size_y)));

    let font = assets.font(&Fonts::Font);
    let style = FontStyle::new(50.0, Color::WHITE);
    let rendered_text = font.render(text, &style)?;

    let color = Color::from_hex("#00FF71");
    let hover_color = color.multiply(Color::from_hex(&"#858585"));

    Ok(ButtonConfig {
        text: rendered_text,
        background: "test_button".into(),
        text_location: button_location,
        background_location: location,
        blend_color: Some(color),
        hover_color: Some(hover_color),
    })
}
