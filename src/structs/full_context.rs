use crate::{
    assets::loaded::Images,
    states::OpenWindow,
    structs::{camera_work::CameraWork, point::Point, simple_context::SimpleContext},
};
use quicksilver::{
    geom::Vector,
    graphics::Color,
    prelude::Background::{Col, Img},
};

pub struct FullContext<'a, 'b: 'a> {
    pub simple_context: &'a mut SimpleContext<'b>,
    pub cam_works: &'a mut CameraWork,
    next_screen: Option<OpenWindow>,
}
impl<'a, 'b: 'a> FullContext<'a, 'b> {
    pub fn new(cam_works: &'a mut CameraWork, simple_context: &'a mut SimpleContext<'b>) -> Self {
        Self {
            cam_works,
            next_screen: None,
            simple_context,
        }
    }
    pub fn draw_full_square_on_grid(&mut self, loc: &Point, color: Color) {
        let rec = self.cam_works.pos_to_full_square_on_grid(loc);
        self.simple_context.draw(&rec, Col(color));
    }
    pub fn draw_tile_on_grid(&mut self, loc: &Point, tile: &str) {
        let img = self
            .simple_context
            .assets
            .modules
            .get_image_by_tile_type(tile);
        let rec = self.cam_works.pos_to_full_square_on_grid(loc);
        self.simple_context.draw(&rec, Img(img));
    }
    pub fn draw_image_on_grid(&mut self, loc: &Point, image: Images) {
        let rec = self.cam_works.pos_to_full_square_on_grid(loc);
        self.simple_context
            .draw(&rec, Img(self.simple_context.assets.image(&image)));
    }
    pub fn get_outer_cell_points(&self) -> (Point, Point) {
        self.cam_works.get_outer_cell_points()
    }
    pub fn screen_to_grid(&self, pos: Vector) -> Option<Point> {
        self.cam_works.screen_to_grid(pos)
    }
    pub fn set_next_screen(&mut self, next_screen: Option<OpenWindow>) {
        self.next_screen = next_screen;
    }
    pub fn get_next_screen(&self) -> Option<OpenWindow> {
        self.next_screen
    }
}
