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
#[derive(PartialEq)]
enum NextPath {
	Left,
	Right,
	Up,
	Down,
	None
}
pub struct Character {
	_name : String,
	location : Point,
	point_of_interest : Point,
	time_until_new : usize,
	walk_speed : usize,
	time_till_walk : usize,
	next_path : NextPath
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
			time_till_walk : 0,
			next_path : NextPath::None
		}
	}
	pub fn calc_path(&mut self, grid : &Field, rng : &mut ThreadRng) {
		//println!("{},{}",self.point_of_interest.x,self.point_of_interest.y);
		
		self.next_path = NextPath::None;
		
		let new_pos =
			if self.location.x != self.point_of_interest.x {
				self.walk_horizontal()
			} else if self.location.y != self.point_of_interest.y {
				self.walk_vertical()
			} else if rng.gen() {
				self.walk_horizontal()
			} else {
				self.walk_vertical()
			};
		if self.check_walkable_tile(grid, &new_pos.1) {
			self.next_path = new_pos.0;
		} else {
			match new_pos.0 {
				NextPath::Down=> println!("Down"),
				NextPath::Up=> println!("Up"),
				NextPath::Left=> println!("Left"),
				NextPath::Right=> println!("Right"),
				NextPath::None=> println!("None"),
			}
			if new_pos.0 == NextPath::None {
				println!("Its none");
				return
			}
			let new_point = 
				if new_pos.0 == NextPath::Left || new_pos.0 == NextPath::Right {
					self.walk_vertical()
				} else {
					self.walk_horizontal()
				};
			
			if self.check_walkable_tile(grid, &new_point.1) {
				self.next_path = new_point.0
			} else {
				self.next_path = NextPath::None
			}
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
		self.time_till_walk = self.walk_speed * self.get_walk_speed_penalty(grid);
		match self.next_path {
			NextPath::Down  => self.location.y += 1,
			NextPath::Up    => self.location.y -=1,
			NextPath::Left  => self.location.x -=1,
			NextPath::Right => self.location.x +=1,
			NextPath::None  => {}
		}
		
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
	fn walk_horizontal(&mut self) -> (NextPath,Point) {
		let mut new_pos = self.location;
		let dir = 
			if new_pos.x < self.point_of_interest.x {
				new_pos.x += 1;
				NextPath::Right
			} else if new_pos.x == 0 {
				NextPath::None
			} else {
				new_pos.x -= 1;
				NextPath::Left
			};
		(dir,new_pos)
	}
	fn walk_vertical(&mut self) -> (NextPath,Point) {
		let mut new_pos = self.location;
		let dir = 
			if new_pos.y < self.point_of_interest.y {
				new_pos.y += 1;
				NextPath::Down
			} else if new_pos.y == 0 {
				NextPath::None
			} else {
				new_pos.y -= 1;
				NextPath::Up
			};
		(dir,new_pos)
	}
	fn check_walkable_tile(&self, grid : &Field, point : &Point) -> bool {
		match &grid.get_cell(point) {
			None => false,
			Some(feature) => match &feature.feature {
				None => true,
				Some(feature) => match feature {
					CellFeature::Wall => false
				}
			}
		}
	}
}