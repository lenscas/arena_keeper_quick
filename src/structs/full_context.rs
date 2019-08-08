use quicksilver::geom::Vector;
use quicksilver::input::Mouse;
use crate::generated::assets::loaded::Images;
use crate::generated::assets::loaded::AssetManager;
use crate::structs::camera_work::CameraWork;
use crate::structs::gui_2::Context;
use crate::structs::gui_2::Interaction;
use crate::structs::gui_2::Widget;
use crate::structs::point::Point;
use quicksilver::graphics::Color;
use quicksilver::lifecycle::Window;

pub struct FullContext<'a> {
    window: &'a mut Window,
    gui: Context<'a>,
    cam_works: &'a mut CameraWork,
    assets: &'a AssetManager,
}
impl<'a> FullContext<'a> {
    pub fn new(
        window: &'a mut Window,
        gui: Context<'a>,
        cam_works: &'a mut CameraWork,
        assets: &'a AssetManager,
    ) -> Self {
        Self {
            window,
            gui,
            cam_works,
            assets,
        }
    }
    pub fn get_gui(&mut self) -> &mut Context<'a> {
        &mut self.gui
    }
    pub fn get_assets(&self) -> &AssetManager {
        self.assets
    }
    pub fn draw_full_square_on_grid(&mut self, loc: &Point, color: Color) {
        self.cam_works
            .draw_full_square_on_grid(loc, color, &mut self.window);
    }
    pub fn draw_image_on_square(&mut self, loc: &Point, image: Images) {
        self.cam_works
            .draw_image_on_square(loc, self.assets.image(&image), &mut self.window);
    }
    pub fn render_gui(&mut self) {
        self.gui.render(self.window)
    }
    pub fn push_widget(&mut self, widget: impl Widget + 'a) -> Interaction {
        self.gui.push(widget, self.window)
    }
    pub fn get_outer_cell_points(&self) -> (Point,Point){
        self.cam_works.get_outer_cell_points()
    }
    pub fn mouse(&self) -> Mouse {
        self.window.mouse()
    }
    pub fn screen_to_grid(&self,pos : Vector) -> Option<Point> {
        self.cam_works.screen_to_grid(pos)
    }
}
