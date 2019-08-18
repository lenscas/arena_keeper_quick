use quicksilver::prelude::Background::Img;
use quicksilver::prelude::Background::Col;
use quicksilver::graphics::Background;
use quicksilver::graphics::Drawable;
use crate::states::game_state::OpenWindow;
use quicksilver::geom::{Transform,Vector};
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
    next_screen : Option<OpenWindow>,
    current_z : u32
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
            next_screen : None,
            current_z : 0
        }
    }
    pub fn get_gui(&mut self) -> &mut Context<'a> {
        &mut self.gui
    }
    pub fn get_assets(&self) -> &AssetManager {
        self.assets
    }
    pub fn draw_full_square_on_grid(&mut self, loc: &Point, color: Color) {
        let rec = self.cam_works.pos_to_full_square_on_grid(loc);
        self.draw(&rec,Col(color));
    }
    pub fn draw_image_on_grid(&mut self, loc: &Point, image: Images) {
        let rec = self.cam_works.pos_to_full_square_on_grid(loc);
        self.draw(&rec, Img(self.assets.image(&image)));
    }
    pub fn draw(&mut self, draw : &impl Drawable, bkg : Background<'a>) {
        self.current_z +=1;
        self.window.draw_ex(draw, bkg,Transform::rotate(0),self.current_z);
    }
    pub fn render_gui(&mut self) {
        self.gui.render(self.window)
    }
    pub fn get_interaction(&self, widget: &'a mut impl Widget) -> Interaction {
        self.gui.get_interaction(widget, self.window)
    }
    pub fn push_widget(&mut self, widget: impl Widget + 'a) {
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
    pub fn set_next_screen(&mut self, next_screen : Option<OpenWindow>) {
        self.next_screen = next_screen;
    }
    pub fn get_next_screen(&self) -> Option<OpenWindow> {
        self.next_screen
    }
}
