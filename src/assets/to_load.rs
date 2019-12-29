use super::loaded::AssetManager;
use super::loaded::Fonts;
use crate::modules::handle_files::load_everything;
use quicksilver::graphics::Font;
use quicksilver::graphics::Image;
use quicksilver::Error;
use quicksilver::Future;

pub fn load_all() -> impl Future<Item = AssetManager, Error = Error> {
    let mut manager = AssetManager::new();
    Box::new(
        Font::load("font.ttf")
            .map(|v| {
                manager.insert_font(Fonts::Font, v);
                manager
            })
            .and_then(load_everything)
            .and_then(|mut manager| {
                Image::load("test_button.png").map(|v| {
                    manager.insert_image("test_button".to_string(), v);
                    manager
                })
            })
            .and_then(|mut manager| {
                Image::load("square.png").map(|v| {
                    manager.insert_image("square".to_string(), v);
                    manager
                })
            })
            .and_then(|mut manager| {
                Image::load("input_box.png").map(|v| {
                    manager.insert_image("input_box".to_string(), v);
                    manager
                })
            })
            .and_then(|mut manager| {
                Image::load("open_dropdown.png").map(|v| {
                    manager.insert_image("open_dropdown".to_string(), v);
                    manager
                })
            }),
    )
}
