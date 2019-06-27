

use crate::structs::point::Point;
use crate::structs::cell::CellFeature;
use crate::structs::point::PointWithItem;
use crate::structs::cell::CellType;
use crate::structs::cell::Cell;
use noise::Value;
use noise::utils::NoiseMapBuilder;
use noise::utils::PlaneMapBuilder;
use noise::Seedable;

/// A structure that contains the map
pub struct Field {
    grid : Vec<Cell>,
    pub len : usize,
    pub height : usize
}
impl Field {
    pub fn new(len : usize, height : usize, seed : u32) -> Self {
        let mut grid = Vec::<Cell>::new();
        let amount = len * height;
        grid.reserve(amount);
        let noise_gen = Value::new().set_seed(seed);
        let map = PlaneMapBuilder::new(&noise_gen).set_size(len,height).build();
        for cell_spot in 0 ..= amount {
            let num = map.get_value( cell_spot % len, cell_spot / len);
            let cell_type = CellType::from(num);
            let cell = Cell {
                cell_type,
                loc : ((cell_spot % len),(cell_spot / len)).into(),
                feature : None
            };
            grid.insert(cell_spot, cell);

        }
        Self {
            grid,
            len,
            height
        }
    }
    /// used to calculate the index for the cell position in the list using an X,Y coordinate
    fn calc_cell(&self, point : &Point) -> usize {
        let mut x = point.x;
        let mut y = point.y;
        if x >= self.len {
            x = self.len - 1
        }

        if y >= self.height {
            y = self.height - 1
        };
        (y * self.len ) + x
    }
    /// Gets every cell between 2 points. Used to only render the cells that are on the screen.
    pub fn get_part(&self, start : Point, end : Point) -> Vec<Cell> {
        let to_start = self.calc_cell(&start);
        let to_end = self.calc_cell(&end);
        let first_split = self.grid.split_at(to_start).1;
        let second_split = first_split.split_at(1 + to_end - to_start).0;
        second_split
            .iter()
            .filter(
                |v|
                    v.loc.x >= start.x &&
                    v.loc.x <= end.x   &&
                    v.loc.y >= start.y &&
                    v.loc.y <= end.y
            )
            .cloned()
            .collect()
    }
    pub fn add_feature_to_cells(&mut self, cells : Vec<PointWithItem<CellFeature>>) {
        cells.iter().for_each(|v| {
            let place = self.calc_cell(&v.into());
            if let Some(cell) = self.grid.get_mut(place) {
                cell.feature = Some(v.item.clone());
            }

        });
    }
    pub fn get_cell(&self, point : &Point) -> Option<&Cell> {
        let index = self.calc_cell(point);
        self.grid.get(index)
    }
}