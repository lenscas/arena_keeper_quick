use crate::structs::BuyableCharacter;
use crate::structs::{
	CameraWork,
	grid::Field,
	Character
};
use quicksilver::lifecycle::Window;


#[derive(Default)]
pub struct Characters {
	characters : Vec<Character>,
}
impl Characters {
	pub fn new() -> Self{
		Self {
			characters : Vec::new()
		}
	}
	#[cfg(target_arch = "wasm32")]
	fn update_paralel(&mut self, grid : &Field) {
		self.characters.iter_mut().for_each(
			|v| v.update_par(grid)

		);
	}
	#[cfg(not(target_arch = "wasm32"))]
	fn update_paralel(&mut self, grid : &Field) {
		use rayon::prelude::*;
		self.characters.par_iter_mut().for_each(

			|v| v.update_par(grid)

		);
	}
	pub fn update(&mut self, grid : &Field) {
		self.update_paralel(grid);
		self.characters.iter_mut().for_each(|v|v.update(grid));
	}
	pub fn render(&mut self, cam : &CameraWork, window : &mut Window) {
		self.characters.iter_mut().for_each(|v|v.render(cam,window));
	}
	pub fn add_character(&mut self, new_char : BuyableCharacter) {
		self.characters.push(Character::from_bought_char(new_char));
	}
}