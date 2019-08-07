use quicksilver::prelude::Background::Img;
use quicksilver::geom::Vector;
use crate::structs::gui_2::context::Widget;
use quicksilver::geom::Rectangle;
use crate::structs::gui_2::finalize::Interaction;
use quicksilver::lifecycle::Window;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::graphics::Image;
use quicksilver::Result;

#[derive(Clone)]
pub struct State
{
    pub normal: Image,
    pub hovered: Image,
    pub active: Image,
    pub location : Rectangle,
}

impl State
{
    pub fn new_single_raw(image: Image,location : Rectangle ) -> Self 
    {
        let normal = image.clone();
        let hovered = image.clone();
        let active = image;
        Self {
            normal,
            hovered,
            active,
            location,
        }
    }
    pub fn new(
        font: Font,
        style: &FontStyle,
        normal: &str,
        hover: &str,
        active: &str,
        location : Rectangle,
    ) -> Result<Self>
    {
        let normal = font.render(normal, &style)?;
        let hovered = font.render(hover, &style)?;
        let active = font.render(active, &style)?;
        Ok(Self {
            normal,
            hovered,
            active,
            location,
        })
    }
    pub fn new_single_text(font: Font, style: &FontStyle, text: &str, location : Rectangle) -> Result<Self> 
    {
        let normal = font.render(text, &style)?;
        let hovered = font.render(text, &style)?;
        let active = font.render(text, &style)?;

        Ok(Self {
            normal,
            hovered,
            active,
            location
        })
    }
}
impl Widget for State {
    fn render(&self, window : &mut Window, interaction : Interaction) {
        match interaction {
            Interaction::None => window.draw(&self.location,Img(&self.normal)),
            Interaction::Hover => window.draw(&self.location,Img(&self.hovered)),
            Interaction::Clicked => window.draw(&self.location,Img(&self.active))
        }
    }
    fn contains(&self,point: Vector) -> bool {
        point.x >= self.location.pos.x &&
        point.y >= self.location.pos.y &&
        point.x <= self.location.pos.x + self.location.size.x &&
        point.y <= self.location.pos.y + self.location.size.y
    }
}