use crate::assets::loaded::{AssetManager, Images};
use quicksilver::{
    geom::{Rectangle, Transform},
    graphics::{Background, Drawable},
    input::{Keyboard, Mouse},
    lifecycle::Window,
    prelude::Background::Img,
};

use mergui::Context;
pub struct SimpleContext<'a, 'b> {
    pub window: &'a mut Window,
    pub gui: &'a mut Context<'b>,
    pub assets: &'a AssetManager,
    current_z: u32,
}
impl<'a, 'b> SimpleContext<'a, 'b> {
    pub fn new(window: &'a mut Window, gui: &'a mut Context<'b>, assets: &'a AssetManager) -> Self {
        Self {
            window,
            gui,
            assets,
            current_z: 0,
        }
    }
    pub fn get_gui(&mut self) -> &mut Context<'b> {
        &mut self.gui
    }
    pub fn get_assets(&self) -> &AssetManager {
        self.assets
    }
    pub fn draw(&mut self, draw: &impl Drawable, bkg: Background<'a>) {
        self.current_z += 1;
        self.window
            .draw_ex(draw, bkg, Transform::IDENTITY, self.current_z);
    }
    pub fn draw_image(&mut self, place: &Rectangle, image: Images) {
        self.draw(place, Img(self.assets.image(&image)));
    }
    pub fn mouse(&self) -> Mouse {
        self.window.mouse()
    }
    pub fn keyboard(&self) -> &Keyboard {
        self.window.keyboard()
    }
    pub fn get_z(&mut self) -> u32 {
        self.current_z += 1;
        self.current_z
    }
}
