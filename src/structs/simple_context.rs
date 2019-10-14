use crate::{
    assets::loaded::{AssetManager, Images},
    structs::gui_2::{Context, Interaction, Widget},
};
use quicksilver::{
    geom::{Rectangle, Transform},
    graphics::{Background, Drawable},
    input::{Keyboard, Mouse},
    lifecycle::Window,
    prelude::Background::Img,
};
pub struct SimpleContext<'a> {
    window: &'a mut Window,
    gui: Context<'a>,
    pub assets: &'a AssetManager,
    current_z: u32,
}
impl<'a> SimpleContext<'a> {
    pub fn new(window: &'a mut Window, gui: Context<'a>, assets: &'a AssetManager) -> Self {
        Self {
            window,
            gui,
            assets,
            current_z: 0,
        }
    }
    pub fn get_gui(&mut self) -> &mut Context<'a> {
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
    pub fn render_gui(&mut self) {
        self.gui.render(self.window, &mut self.current_z)
    }
    pub fn get_interaction(&self, widget: &'a mut impl Widget) -> Interaction {
        self.gui.get_interaction(widget, self.window)
    }
    pub fn push_widget(&mut self, widget: impl Widget + 'a) {
        self.gui.push(widget)
    }
    pub fn mouse(&self) -> Mouse {
        self.window.mouse()
    }
    pub fn keyboard(&self) -> &Keyboard {
        self.window.keyboard()
    }
}
