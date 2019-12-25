use crate::structs::FullContext;
use quicksilver::Result;

pub struct Grid {}
impl Grid {
    pub fn render(context: &mut FullContext) -> Result<()> {
        let (start, end) = context.get_outer_cell_points();
        let part = context.state.grid.get_part(start, end);
        part.iter().enumerate().for_each(|v| {
            let to_draw =
                v.1.feature
                    .as_ref()
                    .map(|feature| {
                        let feature = context
                            .simple_context
                            .assets
                            .modules
                            .get_feature(feature.get_feature_name());
                        if feature.is_transparent {
                            context.draw_tile_on_grid(&v.1.loc, &v.1.cell_type);
                        }
                        &feature.image
                    })
                    .unwrap_or_else(|| &v.1.cell_type);
            context.draw_tile_on_grid(&v.1.loc, to_draw)
        });
        Ok(())
    }
}
