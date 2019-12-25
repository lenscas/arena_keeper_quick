use super::{grid::Field, point::Point};
use crate::modules::structs::ModulesContainer;
use crate::{
    assets::loaded::Images,
    modules::structs::SpeciesType,
    structs::{BuyableCharacter, CameraWork, SimpleContext},
};
use pathfinding::{directed::astar::astar, prelude::absdiff};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub type CharId = usize;

#[derive(Serialize, Deserialize)]
pub struct Character {
    id: CharId,
    _name: String,
    location: Point,
    point_of_interest: Point,
    time_until_new: usize,
    walk_speed: usize,
    time_till_walk: usize,
    path: Option<VecDeque<Point>>,
    time_until_recalc: usize,
    species: SpeciesType,
    image: Images,
}

impl Character {
    pub fn from_bought_char(id: CharId, bought_char: BuyableCharacter) -> Self {
        Self {
            id,
            location: (1, 1).into(),
            _name: bought_char.get_name(),
            walk_speed: bought_char.get_speed(),
            point_of_interest: (10, 10).into(),
            time_until_new: 500,
            time_till_walk: 0,
            path: None,
            time_until_recalc: 0,
            species: bought_char.get_species(),
            image: bought_char.get_image(),
        }
    }
    /// This function updates everything that multiple characters can do at the same time.
    /// Always execute update_par before update
    pub fn update_par(&mut self, grid: &Field, modules: &ModulesContainer) {
        if self.path.is_none() {
            self.calc_path(grid, modules);
        } else if self.time_until_recalc == 0 {
            self.calc_path(grid, modules);
            if let Some(path) = &self.path {
                self.time_until_recalc = path.len() * 10;
            }
        } else {
            self.time_until_recalc -= 1;
        }
    }
    /// calculates the path from the current point to the point_of_interest and stores it inside self.path
    fn calc_path(&mut self, grid: &Field, modules: &ModulesContainer) {
        if (!self.check_walkable_tile(grid, &self.point_of_interest, modules))
            || self.location == self.point_of_interest
        {
            self.time_until_new = 0;
            self.path = None;
            return;
        }
        let path: Option<VecDeque<Point>> = astar(
            &(self.location.x, self.location.y),
            |&var| {
                let point: Point = var.into();
                let mut possibles = Vec::new();
                if point.y > 0 {
                    possibles.push((point.x, point.y - 1));
                }
                if point.x > 0 {
                    possibles.push((point.x - 1, point.y));
                }
                if point.x < grid.len - 1 {
                    possibles.push((point.x + 1, point.y));
                }
                if point.y < grid.height - 1 {
                    possibles.push((point.x, point.y + 1));
                }
                possibles
                    .into_iter()
                    .filter(|v| self.check_walkable_tile(grid, &v.into(), modules))
                    .map(|p| (p, self.calculate_cost(grid, &p.into(), modules)))
            },
            |&(x, y)| absdiff(x, self.point_of_interest.x) + absdiff(y, self.point_of_interest.y),
            |&p| self.point_of_interest == p.into(),
        )
        .map(|v| v.0.iter().map(|b| b.into()).collect());
        self.path = path;
    }
    /// This is similar to update_par but updates everything that can't happen at the same time with other characters.
    /// Always execute update_par before update
    pub fn update(&mut self, grid: &mut Field, modules: &ModulesContainer) {
        let mut rng = rand::thread_rng();
        if self.time_until_new == 0 || self.location == self.point_of_interest {
            self.time_until_new = rng.gen();
            self.point_of_interest = if rng.gen() {
                let id = self.id;
                let bed = grid
                    .find_cell_by(|v| match &v.feature {
                        Some(feature) => feature.is_owned_by(id) && feature.can_sleep(id, modules),
                        _ => false,
                    })
                    .or_else(|| {
                        grid.find_cell_by(|v| match &v.feature {
                            Some(feature) => feature.can_sleep(id, modules),
                            _ => false,
                        })
                        /*
                        if let Some(cell) = cell {
                            let feature = cell.feature.as_mut().unwrap();
                            feature.set_owned(Some(id));
                        }
                        cell*/
                    });
                bed.map(|v| {
                    let cell = v.clone();
                    cell.feature.unwrap().set_owned(Some(id));
                    v.loc
                })
                .unwrap_or_else(|| {
                    (rng.gen_range(0, grid.len), rng.gen_range(0, grid.height)).into()
                })
            } else {
                (rng.gen_range(0, grid.len), rng.gen_range(0, grid.height)).into()
            };
            self.path = None;
        }
        self.time_until_new -= 1;
        if self.time_till_walk > 0 {
            self.time_till_walk -= 1;
            return;
        }
        self.time_till_walk = self.calculate_cost(grid, &self.location, modules);
        match &mut self.path {
            None => {}
            Some(path) => match path.pop_front() {
                None => {
                    self.path = None;
                }
                Some(next) => {
                    if self.check_walkable_tile(grid, &next, modules) {
                        self.location = next;
                    } else {
                        self.path = None;
                    }
                }
            },
        }
    }
    fn calculate_cost(&self, grid: &Field, check_on: &Point, modules: &ModulesContainer) -> usize {
        self.get_walk_speed_penalty(grid, check_on, modules) * self.walk_speed
    }
    fn get_walk_speed_penalty(
        &self,
        grid: &Field,
        check_on: &Point,
        modules: &ModulesContainer,
    ) -> usize {
        grid.get_cell(check_on)
            .map(|cell| {
                cell.feature
                    .as_ref()
                    .and_then(|feature| feature.get_speed_penalty(modules))
                    .unwrap_or_else(|| {
                        self.species.get_speed_on_tile(
                            &modules.all_species,
                            &modules.all_tiles,
                            &cell.cell_type,
                        )
                    })
            })
            .unwrap_or_else(|| panic!("{:?} is out of bounds", check_on))
    }
    /// Renders the character.
    pub fn render(&self, context: &mut SimpleContext, cam: &CameraWork) {
        cam.draw_image_on_grid(&self.location, self.image.clone(), context);
    }

    /// Checks wheter this character can walk on a given tile
    fn check_walkable_tile(&self, grid: &Field, point: &Point, modules: &ModulesContainer) -> bool {
        match grid.get_cell(point) {
            None => false,
            Some(cell) => cell
                .feature
                .as_ref()
                .map(|v| v.can_walk(modules))
                .unwrap_or(true),
        }
    }
}
