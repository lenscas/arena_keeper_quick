use crate::structs::gui_2::context::Widget;
use crate::structs::gui_2::finalize::Interaction;
use quicksilver::geom::Rectangle;
use quicksilver::geom::Vector;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::graphics::Image;
use quicksilver::lifecycle::Window;
use quicksilver::prelude::Background::Img;
use quicksilver::Result;

#[derive(Clone)]
pub struct State {
    pub normal: Image,
    pub hovered: Image,
    pub active: Image,
    pub location: Rectangle,
    interaction : Interaction
}

impl State {
    pub fn new_single_raw(image: Image, location: Rectangle) -> Self {
        let normal = image.clone();
        let hovered = image.clone();
        let active = image;
        Self {
            normal,
            hovered,
            active,
            location,
            interaction : Interaction::None
        }
    }
    pub fn new(
        font: &Font,
        style: &FontStyle,
        normal: &str,
        hover: &str,
        active: &str,
        location: Rectangle,
    ) -> Result<Self> {
        let normal = font.render(normal, &style)?;
        let hovered = font.render(hover, &style)?;
        let active = font.render(active, &style)?;
        Ok(Self {
            normal,
            hovered,
            active,
            location,
            interaction : Interaction::None
        })
    }
    pub fn new_single_text(
        font: Font,
        style: &FontStyle,
        text: &str,
        location: Rectangle,
    ) -> Result<Self> {
        let normal = font.render(text, &style)?;
        let hovered = font.render(text, &style)?;
        let active = font.render(text, &style)?;

        Ok(Self {
            normal,
            hovered,
            active,
            location,
            interaction : Interaction::None
        })
    }
}
impl Widget for State {
    fn render(&self, window: &mut Window) {
        match self.interaction {
            Interaction::None => window.draw(&self.location, Img(&self.normal)),
            Interaction::Hover => window.draw(&self.location, Img(&self.hovered)),
            Interaction::Clicked => window.draw(&self.location, Img(&self.active)),
        }
    }
    fn contains(&self, point: Vector) -> bool {
        point.x >= self.location.pos.x
            && point.y >= self.location.pos.y
            && point.x <= self.location.pos.x + self.location.size.x
            && point.y <= self.location.pos.y + self.location.size.y
    }
    fn set_interaction(&mut self, interaction : Interaction) {
        self.interaction = interaction
    }
}
