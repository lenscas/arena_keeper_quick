

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
                x : (cell_spot % len) as isize ,
                y : (cell_spot / len) as isize,
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
    fn calc_cell(&self, mut x:isize,mut y:isize) -> usize {
        if x >= self.len as isize {
            x = self.len as isize - 1
        }

        if y >= self.height as isize {
            y = self.height as isize - 1
        };
        (( y * (self.len as isize) ) + x) as usize
    }
    /// Gets every cell between 2 points. Used to only render the cells that are on the screen.
    pub fn get_part(&self, x_start:isize, y_start : isize, x_end : isize, y_end : isize ) -> Vec<Cell> {
        let to_start = self.calc_cell(x_start, y_start);
        let to_end = self.calc_cell(x_end + 1, y_end);
        let first_split = self.grid.split_at(to_start).1;
        let second_split = first_split.split_at(1 + to_end - to_start).0;
        second_split
            .iter()
            .filter(
                |v|
                    v.x >= x_start &&
                    v.x <= x_end   &&
                    v.y >= y_start &&
                    v.y <= y_end
            )
            .cloned()
            .collect()
    }
    pub fn add_feature_to_cells(&mut self, cells : Vec<PointWithItem<CellFeature>>) {
        cells.iter().for_each(|v| {
            let place = self.calc_cell(v.x as isize,v.y as isize);
            if let Some(cell) = self.grid.get_mut(place) {
                cell.feature = Some(v.item.clone());
            }

        });
    }
}