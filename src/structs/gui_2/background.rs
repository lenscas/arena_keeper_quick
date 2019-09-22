use crate::{
    generated::assets::loaded::{AssetManager, Fonts},
    structs::gui_2::{button::State, Combined, Interaction, Widget},
};
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Blended, Color, FontStyle, Image},
    lifecycle::Window,
};
#[derive(Clone)]
pub struct ButtonBackground {
    image: Image,
    color: Color,
    location: Rectangle,
    interaction: Interaction,
}

impl Widget for ButtonBackground {
    fn render(&self, window: &mut Window, at: &mut u32) {
        let to_blend_with = if self.interaction != Interaction::None {
            self.color.multiply(Color::from_hex(&"#858585"))
        } else {
            self.color
        };
        window.draw_ex(
            &self.location,
            Blended(&self.image, to_blend_with),
            Transform::IDENTITY,
            *at,
        )
    }
    fn contains(&self, point: Vector) -> bool {
        point.x >= self.location.pos.x
            && point.y >= self.location.pos.y
            && point.x <= self.location.pos.x + self.location.size.x
            && point.y <= self.location.pos.y + self.location.size.y
    }
    fn set_interaction(&mut self, interaction: Interaction) {
        self.interaction = interaction
    }
    fn set_pos(&mut self, pos: Rectangle) {
        self.location = pos;
    }
    fn get_pos(&self) -> &Rectangle {
        &self.location
    }
}
impl ButtonBackground {
    pub fn new_success(
        assets: &AssetManager,
        location: Rectangle,
        text: String,
    ) -> Combined<State, Self> {
        let background_location = Rectangle::new_sized(location.size());
        let skip_x = location.size.x / 100.0 * 15.0;
        let size_x = location.size.x - (skip_x * 2.0);
        let skip_y = location.size.y / 100.0 * 30.0;
        let size_y = location.size.y - (skip_y * 2.0);
        let button_location = Rectangle::new((skip_x, skip_y), (size_x, (size_y)));

        let state = State::new_single_text(
            assets.font(&Fonts::Font),
            &FontStyle::new(50.0, Color::WHITE),
            &text,
            button_location,
        )
        .unwrap();

        let background = Self {
            image: assets.image(&"test_button").clone(),
            color: Color::from_hex(&"#00FF71"),
            location: background_location,
            interaction: Interaction::None,
        };

        Combined::new(state, location, background)
    }
}
