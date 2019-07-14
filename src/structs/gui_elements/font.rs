use crate::structs::gui_elements::Finalize;
use quicksilver::lifecycle::Asset;
use quicksilver::combinators::FutureResult;
use quicksilver::graphics::Color;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::{
  Future,
    combinators::{
		result
	}
};

pub fn load<F: 'static,T: 'static>(font : String, size : f32, color : Color, fun : F ) -> Asset<T>
where
	F : FnOnce(Font,FontStyle) -> FutureResult<T,quicksilver::Error>
{
	 Asset::new(Font::load(font).and_then(
		move |font| {
			let style = FontStyle::new(size, color);
			fun(font,style)
		}
	))
}
pub fn load_from<F: 'static,T: 'static>(font : String, size : f32, color : Color, builder : F) -> Asset<(Font,T)>
where
	F : Finalize<to=T>
{
	load(font,size,color,|font,style| result(builder.to_state(font,style)))
}