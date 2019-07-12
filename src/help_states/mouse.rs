use crate::states::game_state::ClickMode;
use crate::{
    funcs::math::sub_from_highest,
    structs::{
        grid::{CellFeature, Field},
        point::{Point, PointWithItem},
        CameraWork,
    },
};
use quicksilver::{graphics::Color, lifecycle::Window, prelude::MouseButton, Result};

pub struct Mouse<'a> {
    pub cam: &'a CameraWork,
    pub clicked: &'a mut Option<Point>,
    pub grid: &'a mut Field,
    pub selected: &'a mut ClickMode,
}
impl<'a> Mouse<'a> {
    fn draw_wall(
        &mut self,
        window: &mut Window,
        key: quicksilver::input::ButtonState,
        grid_pos: Point,
    ) -> Result<()> {
        if let Some(click_point) = &self.clicked {
            let dif_x = sub_from_highest(grid_pos.x, click_point.x);
            let dif_y = sub_from_highest(grid_pos.y, click_point.y);
            let line = if dif_x > dif_y {
                let point = if click_point.x < grid_pos.x {
                    Point {
                        x: click_point.x,
                        y: click_point.y,
                    }
                } else {
                    Point {
                        x: grid_pos.x,
                        y: click_point.y,
                    }
                };
                point.make_horizontal_line(dif_x)
            } else {
                let point = if click_point.y < grid_pos.y {
                    Point {
                        x: click_point.x,
                        y: click_point.y,
                    }
                } else {
                    Point {
                        x: click_point.x,
                        y: grid_pos.y,
                    }
                };
                point.make_vertical_line(dif_y)
            };
            line.iter()
                .for_each(|v| self.cam.draw_full_square_on_grid(v, Color::WHITE, window));
            if !key.is_down() {
                let line: Vec<PointWithItem<CellFeature>> =
                    line.iter().map(|v| v.add_item(CellFeature::Wall)).collect();
                self.grid.add_feature_to_cells(line);
                *self.clicked = None;
            }
        } else if key.is_down() {
            *self.clicked = Some(grid_pos);
        } else {
            *self.clicked = None;
        }
        Ok(())
    }
    fn draw_bed(
        &mut self,
        _: &mut Window,
        key: quicksilver::input::ButtonState,
        click_pos: Point,
    ) -> Result<()> {
        if key.is_down() {
            self.grid
                .add_feature_to_cell(click_pos.add_item(CellFeature::Bed(None)))
        }
        Ok(())
    }
    pub fn render(&mut self, window: &mut Window) -> Result<()> {
        let mouse = window.mouse();
        let key = mouse[MouseButton::Left];
        if let Some(grid_pos) = self.cam.screen_to_grid(mouse.pos()) {
            self.cam
                .draw_full_square_on_grid(&grid_pos, Color::WHITE, window);

            match self.selected {
                ClickMode::Wall => self.draw_wall(window, key, grid_pos),
                ClickMode::Bed => self.draw_bed(window, key, grid_pos),
            }
        } else {
            Ok(())
        }
    }
}
