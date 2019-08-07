use crate::{
    generated::tiles::CellType,
    structs::point::{Point, PointWithItem},
};

use super::{Cell, CellFeature};

use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    OpenSimplex, Seedable,
};

use rayon::prelude::*;

/// A structure that contains the map
pub struct Field {
    grid: Vec<Cell>,
    pub len: usize,
    pub height: usize,
}
impl Field {
    /// Generates a new grid with the given height and width
    pub fn new(len: usize, height: usize, seed: u32) -> Self {
        let mut grid = Vec::<Cell>::new();
        let amount = len * height;
        grid.reserve(amount);
        let noise_gen = OpenSimplex::new().set_seed(seed);
        let map = PlaneMapBuilder::new(&noise_gen)
            .set_size(len, height)
            .build();
        for cell_spot in 0..=amount {
            let num = map.get_value(cell_spot % len, cell_spot / len);
            let cell_type = CellType::from(num);
            let cell = Cell {
                cell_type,
                loc: ((cell_spot % len), (cell_spot / len)).into(),
                feature: CellFeature::None,
            };
            grid.insert(cell_spot, cell);
        }
        Self { grid, len, height }
    }
    /// used to calculate the index for the cell position in the list using an X,Y coordinate
    fn calc_cell(&self, point: &Point) -> usize {
        let mut x = point.x;
        let mut y = point.y;
        if x >= self.len {
            x = self.len - 1
        }

        if y >= self.height {
            y = self.height - 1
        };
        (y * self.len) + x
    }
    /// Gets every cell between 2 points. Used to only render the cells that are on the screen.
    pub fn get_part(&self, start: Point, end: Point) -> Vec<Cell> {
        let to_start = self.calc_cell(&start);
        let to_end = self.calc_cell(&end);
        let first_split = self.grid.split_at(to_start).1;
        let second_split = first_split.split_at(1 + to_end - to_start).0;
        second_split
            .iter()
            .filter(|v| {
                v.loc.x >= start.x && v.loc.x <= end.x && v.loc.y >= start.y && v.loc.y <= end.y
            })
            .cloned()
            .collect()
    }
    /// Adds a feature to a cell at the given point
    /// # Examples
    /// ```
    /// # use arena::structs::grid::Field;
    /// # use arena::structs::grid::CellFeature;
    /// # use arena::structs::point::Point;
    /// # let mut field = Field::new(10,10,0);
    /// # let point = Point{x : 2,y : 2};
    /// field.add_feature_to_cell(&point.add_item(CellFeature::Wall));
    /// assert!(field.get_cell(&point).unwrap().feature == CellFeature::Wall);
    /// ```
    pub fn add_feature_to_cell(&mut self, cell: &PointWithItem<CellFeature>) {
        let feature = cell.item.clone();
        let place = self.calc_cell(&cell.into());
        if let Some(place) = self.grid.get_mut(place) {
            place.feature = feature
        }
    }
    /// Adds a feature to multiple cells
    /// # Examples
    /// ```
    /// # use arena::structs::grid::Field;
    /// # use arena::structs::grid::CellFeature;
    /// # use arena::structs::point::Point;
    /// # let mut field = Field::new(10,10,0);
    /// # let point1 = Point {
    /// #    x : 2,
    /// #    y : 2
    /// # };
    /// # let point2 = Point {
    /// #    x : 3,
    /// #    y : 3
    /// # };
    /// field.add_feature_to_cells(vec![point1.add_item(CellFeature::Wall),point2.add_item(CellFeature::Wall)]);
    /// assert!(field.get_cell(&point1).unwrap().feature == CellFeature::Wall);
    /// assert!(field.get_cell(&point2).unwrap().feature == CellFeature::Wall);
    pub fn add_feature_to_cells(&mut self, cells: Vec<PointWithItem<CellFeature>>) {
        cells.iter().for_each(|v| self.add_feature_to_cell(v));
    }
    pub fn get_cell(&self, point: &Point) -> Option<&Cell> {
        let index = self.calc_cell(point);
        self.grid.get(index)
    }
    /// Tries to find a cell that matches a predicate.
    /// Note though that the cell that it will find doesn't have to be the first matching one in the grid.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn find_cell_by<F>(&mut self, fun: F) -> Option<Cell>
    where
        F: Fn(&Cell) -> bool + std::marker::Send + std::marker::Sync,
    {
        self.grid.par_iter().find_any(|v| fun(v)).cloned()
    }
    #[cfg(target_arch = "wasm32")]
    pub fn find_cell_by<F>(&mut self, fun: F) -> Option<Cell>
    where
        F: Fn(&Cell) -> bool + std::marker::Send + std::marker::Sync,
    {
        self.grid.iter().find(|v| fun(v)).cloned()
    }
}
