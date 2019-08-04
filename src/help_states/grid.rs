use crate::{
    generated::assets::loaded::AssetManager,
    structs::{
        grid::{CellFeature, Field},
        CameraWork
    }
};
use quicksilver::{graphics::Color, lifecycle::Window, Result};

pub struct Grid<'a> {
    cam: &'a mut CameraWork,
    grid: &'a Field,
}
impl<'a> Grid<'a> {
    pub fn new(cam: &'a mut  CameraWork, grid: &'a Field) -> Self {
        Grid { cam, grid }
    }
    pub fn render(&mut self,assets: &AssetManager ,window: &mut Window) -> Result<()> {
        let (start, end) = self.cam.get_outer_cell_points();
        let part = self.grid.get_part(start, end);
        part.iter().enumerate().for_each(|v| {
            let color = match &v.1.feature {
                CellFeature::Wall => Some(Color::INDIGO),
                CellFeature::Bed(_) => Some(Color::from_rgba(60, 60, 60, 0.5)),
                CellFeature::None => None
            };
            if let Some(color) = color {
                self.cam.draw_full_square_on_grid(&v.1.loc, color, window);
            } else {
                self.cam.draw_image_on_square(&v.1.loc, assets.image(&v.1.cell_type.get_image()) ,window);
            }
            
        });
        Ok(())
    }
}
