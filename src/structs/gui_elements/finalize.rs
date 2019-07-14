use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::Result;

pub trait Finalize {
    type to;
    fn to_state(self, font: Font, style: FontStyle) -> Result<(Font, Self::to)>;
}
