use crate::structs::field::Field;
use crate::structs::cell::{
    CellFeature,
    CellType
};

use crate::structs::camera_work::CameraWork;

use quicksilver::{
    Result,
    graphics::{Color},
    lifecycle::{Window}
};
pub struct Grid<'a> {
    cam : &'a CameraWork,
    grid : &'a Field,
}
impl<'a> Grid<'a> {
    pub fn new(cam : &'a CameraWork, grid : &'a Field) -> Self {
        Grid {
            cam,
            grid,
        }
    }
    pub fn render(&self, window : &mut Window) -> Result<()> {
        /*let cell_size = self.cam.calc_size();
        let height = self.cam.height / cell_size;
        let width = self.cam.width / cell_size;
        let start_x = CameraWork::calc_start(self.cam.cam.x, width);
        let start_y = CameraWork::calc_start(self.cam.cam.y, height);
        let end_x = 1 + start_x + width as isize;
        let end_y = 1 + start_y + height as isize;
        */
        let (start,end) = self.cam.get_outer_cell_points();
        let part = self.grid.get_part(start,end);
        part.iter().enumerate().for_each(
            |v| {
                let color = 
                    match &v.1.feature {
                        Some(feature) => match feature {
                            CellFeature::Wall => Color::INDIGO
                        },
                        None => match v.1.cell_type {
                            CellType::Water  => Color::BLUE,
                            CellType::Ground => Color::ORANGE,
                            CellType::Grass  => Color::GREEN,
                            CellType::Stone  => Color::from_rgba(50,50,50,1.0)
                        }
                    };
                self.cam.draw_full_square_on_grid(&v.1.loc, color, window);
            }
        );
        Ok(())
    }
}