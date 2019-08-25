use crate::structs::FullContext;
use crate::states::game_state::ClickMode;
use crate::{
    funcs::math::sub_from_highest,
    structs::{
        grid::{CellFeature, Field},
        point::{Point, PointWithItem},
    },
};
use quicksilver::{graphics::Color, prelude::MouseButton, input::ButtonState};

pub struct Mouse<'a> {
    pub clicked: &'a mut Option<Point>,
    pub grid: &'a mut Field,
    pub selected: &'a mut ClickMode,
}
impl<'a> Mouse<'a> {
    fn draw_wall(
        &mut self,
        context : &mut FullContext,
        key: quicksilver::input::ButtonState,
        grid_pos: Point,
    ) {
        if !key.is_down() {
            *self.clicked = None;
            return;
        }
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
                .for_each(|v| context.draw_full_square_on_grid(v, Color::WHITE));
            if !key.is_down() {
                let line: Vec<PointWithItem<CellFeature>> =
                    line.iter().map(|v| v.add_item(CellFeature::Wall)).collect();
                self.grid.add_feature_to_cells(line);
                *self.clicked = None;
            }
        }
    }
    fn place_bed(
        &mut self,
        click_pos: Point,
    ) {
        self.grid
            .add_feature_to_cell(&click_pos.add_item(CellFeature::Bed(None)))
    }
    pub fn update(&mut self, context : &mut FullContext) {
        let mouse = context.mouse();
        if mouse[MouseButton::Left] == ButtonState::Pressed {
            let click_point = context.screen_to_grid(mouse.pos());
            match self.selected {
                ClickMode::Wall=> {
                    if self.clicked.is_none() {
                        *self.clicked = click_point
                    }
                },
                ClickMode::Bed => {
                    if let Some(click_point) = click_point {
                        self.place_bed(click_point)
                    }

                }

            }
        }


    }
    pub fn render(&mut self, context : &mut FullContext) {
        let mouse = context.mouse();
        let key = mouse[MouseButton::Left];
        if let Some(grid_pos) = context.screen_to_grid(mouse.pos()) {
            context.draw_full_square_on_grid(&grid_pos, Color::WHITE);
            match self.selected {
                ClickMode::Wall => self.draw_wall(context, key, grid_pos),
                ClickMode::Bed => {}
            }
        }
    }
}
