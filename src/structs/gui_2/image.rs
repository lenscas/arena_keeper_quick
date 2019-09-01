
use crate::structs::gui_2::context::Widget;
use crate::structs::gui_2::finalize::Interaction;
use quicksilver::geom::Rectangle;
use quicksilver::geom::Vector;
use quicksilver::graphics;
use quicksilver::lifecycle::Window;
#[derive(Clone)]
pub struct Image {
    pub img : graphics::Image,
    pub position : Rectangle
}

impl Widget for Image {
    fn render(&self, window: &mut Window, at : &mut u32) { 
        window.draw(&self.position, &self.img);
    }
    fn contains(&self, _: Vector) -> bool { false }
    fn set_interaction(&mut self, _ : Interaction) { }

}
impl Image {
    pub fn new(img : graphics::Image,position : Rectangle) -> Self {
        Self {img,position}
    }
}