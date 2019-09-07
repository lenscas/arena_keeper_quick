use crate::generated::assets::loaded::Images;
use crate::states::OpenWindow;
use crate::structs::camera_work::CameraWork;
use crate::structs::point::Point;
use crate::structs::simple_context::SimpleContext;
use quicksilver::geom::Vector;
use quicksilver::graphics::Color;
use quicksilver::prelude::Background::Col;
use quicksilver::prelude::Background::Img;

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
