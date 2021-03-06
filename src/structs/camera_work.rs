use crate::{
    assets::loaded::Images,
    structs::{point::Point, SimpleContext},
};
use quicksilver::{geom::Rectangle, prelude::Background::Img};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CameraWork {
    pub cam: Point,
    pub scroll: usize,
    pub width: usize,
    pub height: usize,
}
impl CameraWork {
    fn calc_start(cam: usize, line_size: usize) -> usize {
        let halved = line_size / 2;
        if cam < halved || cam == 1 {
            0
        } else {
            let calced = cam - halved;
            if calced <= 1 {
                0
            } else {
                calced - 1
            }
        }
    }
    fn calc_size(&self) -> usize {
        self.scroll / 5
    }
    pub fn grid_to_screen(&self, loc: &Point) -> (f32, f32) {
        let cell_size = self.calc_size() as f32;
        let width = 800. / cell_size;
        let len = 600. / cell_size;
        let x = (loc.x as f32 - (self.cam.x as f32 - width as f32 / 2.)) * cell_size as f32;
        let y = (loc.y as f32 - (self.cam.y as f32 - len as f32 / 2.)) * cell_size as f32;
        (x, y)
    }
    pub fn screen_to_grid(&self, loc: quicksilver::geom::Vector) -> Option<Point> {
        let cell_size = self.calc_size() as f32;
        let x = loc.x / cell_size;
        let x = x + (self.cam.x as f32 - (800.0 / cell_size as f32) / 2.);
        let y = loc.y / cell_size;
        let y = y + (self.cam.y as f32 - (600.0 / cell_size as f32) / 2.);
        if x < 0. || y < 0. {
            None
        } else {
            Some((x as usize, y as usize).into())
        }
    }
    pub fn get_outer_cell_points(&self) -> (Point, Point) {
        let cell_size = self.calc_size();
        let cell_size = if cell_size == 0 { 1 } else { cell_size };
        let height = self.height / cell_size;
        let width = self.width / cell_size;
        let start_x = CameraWork::calc_start(self.cam.x, width);
        let start_y = CameraWork::calc_start(self.cam.y, height);
        let end_x = 1 + start_x + width;
        let end_y = 1 + start_y + height;
        ((start_x, start_y).into(), (end_x, end_y).into())
    }
    pub fn pos_to_full_square_on_grid(&self, loc: &Point) -> Rectangle {
        let screen_pos = self.grid_to_screen(loc);
        let cell_sizef = self.calc_size() as f32;
        Rectangle::new(screen_pos, (cell_sizef, cell_sizef))
    }
    pub fn draw_image_on_grid(&self, loc: &Point, image: Images, context: &mut SimpleContext) {
        let rec = self.pos_to_full_square_on_grid(loc);
        context.draw(&rec, Img(context.assets.image(&image)));
    }
}
