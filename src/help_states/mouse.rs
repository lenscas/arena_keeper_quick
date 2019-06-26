use crate::structs::camera_work::CameraWork;
use crate::funcs::math::sub_from_highest;
use crate::structs::point::{Point,PointWithItem};
use crate::structs::cell::CellFeature;
use crate::structs::field::Field;

use quicksilver::prelude::MouseButton;
use quicksilver::{
    Result,
    graphics::{Color},
    lifecycle::Window
};
pub struct Mouse<'a> {
    cam : &'a CameraWork,
    clicked : &'a mut Option<Point>,
    grid : &'a mut Field
}
impl<'a> Mouse<'a> {
    pub fn new(cam : &'a CameraWork, clicked : &'a mut Option<Point>, grid : &'a mut Field ) -> Self {
        Mouse {
            cam,
            clicked,
            grid
        }
    }
    pub fn render(&mut self, window : &mut Window) -> Result<()> {
        let mouse = window.mouse();
        let key = mouse[MouseButton::Left];
        if let Some(grid_pos) = self.cam.screen_to_grid(mouse.pos()) {
            self.cam.draw_full_square_on_grid(&grid_pos, Color::WHITE, window);
            if let Some(click_point) = &self.clicked {
                let dif_x = sub_from_highest(grid_pos.x, click_point.x);
                let dif_y = sub_from_highest(grid_pos.y, click_point.y);
                let line =
                    if dif_x > dif_y {
                        let point = if click_point.x < grid_pos.x {
                            Point {x : click_point.x,y: click_point.y}
                        } else {
                            Point {x : grid_pos.x,y: click_point.y}
                        };
                        point.make_horizontal_line(dif_x)
                    } else {
                        let point = if click_point.y < grid_pos.y {
                            Point {x : click_point.x,y: click_point.y}
                        } else {
                            Point {x : click_point.x,y: grid_pos.y}
                        };
                        point.make_vertical_line(dif_y)
                    };
                line.iter().for_each(|v| self.cam.draw_full_square_on_grid(v, Color::WHITE, window));
                if !key.is_down() {
                    let line :Vec<PointWithItem<CellFeature>> = line.iter().map(|v|v.add_item(CellFeature::Wall)).collect();
                    self.grid.add_feature_to_cells(line);
                    *self.clicked = None;
                }
            } else if key.is_down() {
                *self.clicked = Some(grid_pos);
            } else {
                *self.clicked = None;
            }
        } else {
            *self.clicked = None;
        }
        Ok(())
    }
}