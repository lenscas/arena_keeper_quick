use super::{
	grid::{
		Field,
		CellType,
		CellFeature
	},
	point::Point,
	CameraWork,

};

use quicksilver::{
	lifecycle::Window,
	graphics::Color
};
use rand::{
	prelude::*,
	distributions::Alphanumeric
};

pub struct Character {
	_name : String,
	location : Point,
	point_of_interest : Point,
	time_until_new : usize,
	walk_speed : usize,
	time_till_walk : usize
}
impl Default for Character {
    fn default() -> Self {
        Self::new()
    }
}

impl Character {
	pub fn new() -> Self {
		let mut rng = rand::thread_rng();
		let s = rng
			.sample_iter(&Alphanumeric)
			.take(10)
			.collect::<String>();
		Self {
			_name : s,
			location : (1,1).into(),
			point_of_interest : (3,3).into(),
			time_until_new : 500,
			walk_speed : rng.gen_range(1,8),
			time_till_walk : 0
		}
	}
	pub fn update(&mut self, grid : &Field) {
		let mut rng = rand::thread_rng();
		if self.time_until_new == 0 || self.location == self.point_of_interest {
			self.time_until_new = rng.gen();
			self.point_of_interest = (
				rng.gen_range(0,grid.len),
				rng.gen_range(0,grid.height)
			).into()
		}
		self.time_until_new -= 1;
		if self.time_till_walk > 0 {
			self.time_till_walk -= 1;
			return
		}
		let new_pos =
			if self.location.x == self.point_of_interest.x {
				(true,self.walk_horizontal())
			} else if self.location.y == self.point_of_interest.y {
				(false,self.walk_vertical())
			} else if rng.gen() {
				(true,self.walk_horizontal())
			} else {
				(false,self.walk_vertical())
			};
		if self.check_walkable_tile(grid, &new_pos.1) {
			self.location = new_pos.1;
		} else {
			let new_point = if new_pos.0 {
				self.walk_vertical()
			} else {
				self.walk_horizontal()
			};
			if self.check_walkable_tile(grid, &new_point) {
				self.location = new_point
			}
		}
		self.time_till_walk = self.walk_speed * self.get_walk_speed_penalty(grid);
	}
	pub fn get_walk_speed_penalty(&self, grid : &Field) -> usize {
		if let Some(cell) = grid.get_cell(&self.location) {
			match cell.cell_type {
				CellType::Water => 10,
				CellType::Grass => 1,
				CellType::Ground => 2,
				CellType::Stone => 3
			}
		} else {
			unreachable!()
		}
	}
	pub fn render(&self, cam : &CameraWork, window : &mut Window) {
		cam.draw_full_square_on_grid(&self.location, Color::BLACK, window);
	}
	fn walk_horizontal(&mut self) -> Point {
		let mut new_pos = self.location;
		if new_pos.x < self.point_of_interest.x {
			new_pos.x += 1;
		} else {
			new_pos.x -= 1;
		}
		new_pos
	}
	fn walk_vertical(&mut self) -> Point {
		let mut new_pos = self.location;
		if self.location.y < self.point_of_interest.y {
			new_pos.y += 1;
		} else {
			new_pos.y -= 1;
		}
		new_pos
	}
	fn check_walkable_tile(&self, grid : &Field, point : &Point) -> bool {
		match &grid.get_cell(point).unwrap().feature {
			None => true,
			Some(feature) => match feature {
				CellFeature::Wall => false
			}
		}
	}
}