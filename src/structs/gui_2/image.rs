use crate::structs::gui_2::{context::Widget, finalize::Interaction};
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics,
    lifecycle::Window,
};
#[derive(Clone)]
pub struct Image {
    pub img: graphics::Image,
    pub position: Rectangle,
}

impl Widget for Image {
    fn render(&self, window: &mut Window,at: &mut u32) {
        window.draw_ex(&self.position, &self.img, Transform::IDENTITY, *at);
    }
    fn contains(&self, _: Vector) -> bool {
        false
    }
    fn set_interaction(&mut self, _: Interaction) {}
    fn get_pos(&self) -> &Rectangle {
        &self.position
    }
    fn set_pos(&mut self, pos : Rectangle) {
        self.position = pos;
    }
}
impl Image {
    pub fn new(img: graphics::Image, position: Rectangle) -> Self {
        Self { img, position }
    }
}
