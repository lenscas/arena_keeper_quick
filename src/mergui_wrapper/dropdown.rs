use crate::assets::loaded::{AssetManager, Fonts};
use mergui::widgets::{DropDownConfig, DropDownValueConfig};
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, FontStyle},
    Result,
};
use std::marker::PhantomData;

pub struct SimpleDropDownConfig<T: Clone> {
    pub location: Rectangle,
    pub values: Vec<(T, String)>,
    pub selected: usize,
    pub button_width: f32,
}
pub fn drop_down<T: Clone>(
    assets: &AssetManager,
    config: SimpleDropDownConfig<T>,
) -> Result<DropDownConfig<T, DropDownValueConfig<T>>> {
    let font = assets.font(&Fonts::Font);
    let normal = FontStyle::new(50.0, Color::BLACK);
    let hover = FontStyle::new(50.0, Color::BLUE);

    let mut new_values = Vec::new();
    for v in config.values {
        new_values.push(DropDownValueConfig {
            value: v.0,
            normal: font.render(&v.1, &normal)?,
            hover: Some(font.render(&v.1, &hover)?),
        });
    }

    Ok(DropDownConfig {
        location: config.location,
        open_button: "open_dropdown".into(),
        open_button_size: (config.button_width, config.location.size.y).into(),
        option_height: config.location.size.y,
        selected: Some(config.selected),
        values: new_values,
        t: PhantomData,
        divider_color: Color::BLACK,
        divider_size: 2f32,
    })
}
