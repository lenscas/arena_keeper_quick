use crate::structs::{
    grid::{CellFeature, Field},
    FullContext,
};
use quicksilver::{graphics::Color, Result};

pub struct Grid<'a> {
    grid: &'a Field,
}
impl<'a> Grid<'a> {
    pub fn new(grid: &'a Field) -> Self {
        Grid { grid }
    }
    pub fn render(&mut self, context: &mut FullContext) -> Result<()> {
        let (start, end) = context.get_outer_cell_points();
        let part = self.grid.get_part(start, end);
        part.iter().enumerate().for_each(|v| {
            let color = match &v.1.feature {
                CellFeature::Wall => Some(Color::INDIGO),
                CellFeature::Bed(_) => Some(Color::from_rgba(60, 60, 60, 0.5)),
                CellFeature::None => None,
            };
            if let Some(color) = color {
                context.draw_full_square_on_grid(&v.1.loc, color);
            } else {
                context.draw_image_on_grid(&v.1.loc, v.1.cell_type.get_image());
            }
        });
        Ok(())
    }
}
