use pathfinding::prelude::absdiff;
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
use pathfinding::directed::astar::astar;


pub struct Character {
	_name : String,
	location : Point,
	point_of_interest : Point,
	time_until_new : usize,
	walk_speed : usize,
	time_till_walk : usize,
	path : Option<std::collections::VecDeque<Point>>,
	time_until_recalc : usize
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
			point_of_interest : (10,10).into(),
			time_until_new : 500,
			walk_speed : rng.gen_range(1,8),
			time_till_walk : 0,
			path : None,
			time_until_recalc : 0
		}
	}
	pub fn update_par(&mut self, grid : &Field){
		if self.path.is_none() {
			self.calc_path(grid);
		} else if self.time_until_recalc == 0 {
			self.calc_path(grid);
			if let Some(path) = &self.path {
				self.time_until_recalc = path.len() * 10;
			}
		} else {
			self.time_until_recalc -= 1;
		}
	}
	fn calc_path(&mut self, grid : &Field) {
		self.path = 
			astar(
				&(self.location.x,self.location.y),
				|&var| {
					let point : Point = var.into(); 
					
					let mut possibles = vec![
						(point.x,point.y+1),
						(point.x+1,point.y),
					];
					if point.y > 0 {
						possibles.push((point.x,point.y-1));	
					}
					if point.x > 0 {
						possibles.push((point.x - 1,point.y));
					}
					possibles.into_iter()
					.filter(|v| self.check_walkable_tile(grid, &v.into()))
					.map(|p| (p, self.calculate_cost(grid,&p.into())))
				},
				|&(x, y)| absdiff(x, self.point_of_interest.x) + absdiff(y, self.point_of_interest.y),
				|&p| self.point_of_interest == p.into()
			)
			.map(
				|v| 
					v.0.iter()
					.map(
						|b| b.into()
					)
					.collect()
			);
	}
	pub fn update(&mut self, grid : &Field) {
		
		let mut rng = rand::thread_rng();
		if self.time_until_new == 0 || self.location == self.point_of_interest {
			self.time_until_new = rng.gen();
			self.point_of_interest = (
				rng.gen_range(0,grid.len),
				rng.gen_range(0,grid.height)
			).into();
			self.path = None;
		}
		self.time_until_new -= 1;
		if self.time_till_walk > 0 {
			self.time_till_walk -= 1;
			return
		}
		self.time_till_walk = self.calculate_cost(grid, &self.location);
		match &mut self.path {
			None => {
			},
			Some(path) => {
				match path.pop_front() {
					None => {
					},
					Some(next) => {
						if self.check_walkable_tile(grid, &next) {
							self.location = next;
						} else {
							self.path = None;
						}
					}
				}
			}
		}
		
	}
	fn calculate_cost(&self, grid : &Field, check_on : &Point) -> usize {
		self.get_walk_speed_penalty(grid, check_on) * self.walk_speed
	}
	pub fn get_walk_speed_penalty(&self, grid : &Field, check_on : &Point) -> usize {
		if let Some(cell) = grid.get_cell(check_on) {
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