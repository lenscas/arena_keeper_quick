export const makeFile = (enums : string[]) => `
use std::collections::HashMap;
use quicksilver::graphics::Font;
use quicksilver::graphics::Image;
${enums.join("\n")}

#[derive(Default)]
pub struct AssetManager {
	images : HashMap<Images, Image>,
	fonts : HashMap<Fonts, Font>
}

impl AssetManager {
	pub fn new() -> Self {
		Self {
			images : HashMap::new(),
			fonts : HashMap::new()
		}
	}
	pub fn insert_image(&mut self, at : Images , image : Image) {
		self.images.insert(at, image);
	}
	pub fn image(&self, at : &Images) -> &Image {
		self.images.get(at).unwrap()
	}
	pub fn insert_font(&mut self, at : Fonts, font : Font) {
		self.fonts.insert(at, font);
	}
	pub fn font(&self, at : &Fonts) -> &Font {
		self.fonts.get(at).unwrap()
	}
}
`
export const template = (name : string, values:string[])=> `
#[derive(PartialEq,Eq,Hash)]
pub enum ${name} {
	${values.join(",\n\t")}
}`