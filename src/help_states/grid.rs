use crate::structs::{
    grid::{
        CellFeature,
        CellType,
        Field,
    },
    CameraWork
};
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