use crate::{
    structs::gui_2::{Widget,Interaction,Combined,button::State},
    generated::assets::loaded::{AssetManager,Fonts, Images}
};
use quicksilver::{
    graphics::{Color,Image,Background::Blended,FontStyle},
    geom::{Rectangle,Vector,Transform},
    lifecycle::Window
};
#[derive(Clone)]
pub struct ButtonBackground {
    image : Image,
    color : Color,
    location : Rectangle,
    interaction : Interaction
}

impl Widget for ButtonBackground {
    fn render(&self, window: &mut Window, at : &mut u32) {
        let to_blend_with = if self.interaction != Interaction::None {
            self.color.multiply(Color::from_hex(&"#858585"))
        } else {
            self.color
        };
        window.draw_ex(&self.location, Blended(&self.image,to_blend_with), Transform::IDENTITY,*at)
     }
    fn contains(&self, point: Vector) -> bool { 
        point.x >= self.location.pos.x
            && point.y >= self.location.pos.y
            && point.x <= self.location.pos.x + self.location.size.x
            && point.y <= self.location.pos.y + self.location.size.y 
    }
    fn set_interaction(&mut self, interaction : Interaction) { self.interaction = interaction }

}
impl ButtonBackground {
    pub fn new_success(assets : &AssetManager, location : Rectangle, text : String) -> Combined<State,Self> {
        let background_location = Rectangle::new_sized(location.size());
        let button_location = Rectangle::new((5,5), location.size - (10,10).into());
        let state= State::new_single_text(
            assets.font(&Fonts::Font),
            &FontStyle::new(50.1, Color::WHITE),
            &text,
            button_location
        ).unwrap();
        let background = Self {
            image : assets.image(&Images::TestButton).clone(),
            color : Color::from_hex(&"#00FF71"),
            location : background_location,
            interaction : Interaction::None
            
        };
        
        Combined::new(
            state,
            location,
            background
        )
        
    }
}
