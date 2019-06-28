use crate::structs::{
	CameraWork,
	grid::Field,
	Character
};
use quicksilver::lifecycle::Window;
use rayon::prelude::*;

#[derive(Default)]
pub struct Characters {
	characters : Vec<Character>,
	time_until_spawn : usize
}
impl Characters {
	pub fn new() -> Self{
		Self {
			characters : Vec::new(),
			time_until_spawn : 10
		}
	}
	pub fn update(&mut self, grid : &Field) {
		self.time_until_spawn -= 1;
		if self.time_until_spawn == 0 {
			self.time_until_spawn = 5;
			self.characters.push(Character::new());
		}
		self.characters.par_iter_mut().for_each_init(
			rand::thread_rng,
			|rng,v| v.calc_path(grid,rng)
				
		);
		self.characters.iter_mut().for_each(|v|v.update(grid));
	}
	pub fn render(&mut self, cam : &CameraWork, window : &mut Window) {
		self.characters.iter_mut().for_each(|v|v.render(cam,window));
	}
}