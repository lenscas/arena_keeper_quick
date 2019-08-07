use crate::structs::gui_2::Interaction;
use crate::structs::gui_2::Widget;
use crate::generated::assets::loaded::AssetManager;
use quicksilver::graphics::Image;
use quicksilver::graphics::Color;
use crate::structs::point::Point;
use crate::structs::camera_work::CameraWork;
use crate::structs::gui_2::Context;
use quicksilver::lifecycle::Window;

pub struct FullContext<'a> {
    window : &'a mut Window,
    gui : Context<'a>,
    cam_works : &'a mut CameraWork,
    assets : &'a AssetManager
}
impl<'a> FullContext<'a> {
    pub fn new(window : &'a mut Window,gui : Context<'a>, cam_works : &'a mut CameraWork,assets : &'a AssetManager) -> Self {
        Self {
            window,gui,cam_works,assets
        }
    }
    pub fn get_gui(&mut self) -> &mut Context<'a> {
        &mut self.gui
    }
    pub fn get_assets(&self) -> &AssetManager {
        self.assets
    }
    pub fn draw_full_square_on_grid(&mut self, loc: &Point, color: Color) {
        self.cam_works.draw_full_square_on_grid(loc,color, &mut self.window);
    }
    pub fn draw_image_on_square(&mut self, loc: &Point, image : &Image) {
       self.cam_works.draw_image_on_square(loc,image,&mut self.window);
    }
    pub fn render_gui(&mut self) {
        self.gui.render(self.window)
    }
    pub fn push_widget(&mut self, widget : impl Widget + 'a) -> Interaction
    {
        self.gui.push(widget, self.window)
    }
}