

use crate::structs::cell::CellType;
use crate::structs::cell::Cell;
/// A structure that contains the map
pub struct Field {
    grid : Vec<Cell>,
    pub len : usize,
    pub height : usize
}
impl Field {
    pub fn new(len : usize, height : usize) -> Self {
        let mut grid = Vec::<Cell>::new();
        let amount = len * height;
        grid.reserve(amount);
        for cell_spot in 0 ..= amount {
            let cell_type =
                if (cell_spot / len) % 2 == 0 {
                    if cell_spot % 2 == 0 {
                        CellType::Grass
                    } else {
                        CellType::Water
                    }
                } else if cell_spot % 2 == 0 {
                    CellType::Stone
                } else {
                    CellType::Ground
                };
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
    /// Just a method for testing. Marks the given square as clicked
    pub fn clicked_on(&mut self, x :usize, y:usize){
        let in_arr = y * self.len + x;
        assert!(in_arr < self.grid.len(), format!("Given location ({},{}) does not exist", x,y));
        let cell = self.grid.get_mut(in_arr).unwrap();
        cell.cell_type = CellType::Clicked;
    }
}