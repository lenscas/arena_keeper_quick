use crate::structs::cell::CellFeature;
use crate::structs::point::PointWithItem;
use crate::structs::point::Point;
use crate::funcs::math::sub_from_highest;
use quicksilver::prelude::MouseButton;
use crate::funcs::math::sub_save;
use crate::funcs::controls::check_multiple;
use quicksilver::prelude::Key;
use quicksilver::prelude::Background::Col;
use crate::structs::field::Field;
use crate::structs::cell::CellType;
use quicksilver::{
    Result,
    graphics::{Color},
    lifecycle::{State, Window},
    geom::{Rectangle}
};
pub struct GameState {
    grid : Field,
    cam_x : usize,
    cam_y : usize,
    scroll : usize,
    clicked : Option<(usize,usize)>
}
impl State for GameState {
     fn new() -> Result<Self> {
        Ok(Self {
            grid : Field::new(101,81),
            cam_x : 101/2 + 1,
            cam_y : 81/2 +1,
            scroll : 100,
            clicked : None
        })
    }
    fn update(&mut self, window : &mut Window) -> Result<()> {
        let board = window.keyboard();

        if check_multiple(board,&[Key::Left,Key::A]) {
            self.cam_x = sub_save(self.cam_x,1);
        }
        if check_multiple(board,&[Key::Right,Key::D]) {
            self.cam_x += 1;
        }
        if check_multiple(board,&[Key::Up,Key::W]) {
            self.cam_y = sub_save(self.cam_y,1);
        }
        if check_multiple(board,&[Key::Down,Key::S]) {
            self.cam_y += 1;
        }
        let scroll = window.mouse().wheel().y as isize;
        if scroll > self.scroll as isize {
            self.scroll = 0
        } else {
            let scroll = (self.scroll as isize - scroll) as usize;
            self.scroll = scroll;
        };
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.draw_grid(window)?;
        self.draw_mouse(window)?;
        Ok(())
    }
}
impl GameState {
    fn calc_size (&self) -> usize {
        self.scroll / 5
    }
    fn grid_to_screen(&self, loc : (usize,usize)) -> (f32,f32) {
        let cell_size = self.calc_size() as f32;
        let width = 800. / cell_size;
        let len = 600. / cell_size;
        let x = (loc.0 as f32 - (self.cam_x as f32 - width as f32 / 2.)) * cell_size as f32;
        let y = (loc.1 as f32 - (self.cam_y as f32 - len as f32 / 2.)) * cell_size as f32;
        (x,y)
    }
    fn screen_to_grid(&self, loc : quicksilver::geom::Vector) -> Option<(usize,usize)> {
        let cell_size = self.calc_size() as f32;
        let x = loc.x / cell_size;
        let x = x + (self.cam_x as f32 - (800.0 / cell_size as f32) /2.);
        let y = loc.y / cell_size;
        let y = y + (self.cam_y as f32 - (600.0 / cell_size as f32) / 2.);
        if x <0. || y < 0. {
            None
        } else {
            Some((x as usize, y as usize))
        }

    }
    fn draw_mouse(&mut self, window : &mut Window) -> Result<()> {
        let cell_size = self.calc_size();
        let cell_sizef = cell_size as f32;
        let mouse = window.mouse();
        let key = mouse[MouseButton::Left];
        if let Some(grid_pos) = self.screen_to_grid(mouse.pos()) {
            let screen_pos = self.grid_to_screen(grid_pos);
            window.draw(&Rectangle::new(screen_pos, (cell_sizef, cell_sizef)), Col(Color::WHITE) );
            if let Some(click_point) = self.clicked {
                let dif_x = sub_from_highest(grid_pos.0, click_point.0);
                let dif_y = sub_from_highest(grid_pos.1, click_point.1);
                let line =
                    if dif_x > dif_y {
                        let point = if click_point.0 < grid_pos.0 {
                            Point {x : click_point.0,y: click_point.1}
                        } else {
                            Point {x : grid_pos.0,y: click_point.1}
                        };
                        point.make_horizontal_line(dif_x)
                    } else {
                        let point = if click_point.1 < grid_pos.1 {
                            Point {x : click_point.0,y: click_point.1}
                        } else {
                            Point {x : click_point.0,y: grid_pos.1}
                        };
                        point.make_vertical_line(dif_y)
                    };
                line.iter().for_each(|v| window.draw(&Rectangle::new(self.grid_to_screen((v.x,v.y)),(cell_sizef,cell_sizef)),Col(Color::WHITE)));
                if !key.is_down() {
                    let line :Vec<PointWithItem<CellFeature>> = line.iter().map(|v|v.add_item(CellFeature::Wall)).collect();
                    self.grid.add_feature_to_cells(line);
                    self.clicked = None;
                }
            } else if key.is_down() {
                self.clicked = Some(grid_pos);
            } else {
                self.clicked = None;
            }
        } else {
            self.clicked = None;
        }

        Ok(())
    }
    fn draw_grid(&self, window : &mut Window) -> Result<()> {
        let cell_size = self.calc_size();
        let height = 600 / cell_size;
        let width = 800 / cell_size;
        let start_x = Self::calc_start(self.cam_x, width);
        let start_y = Self::calc_start(self.cam_y, height);
        let end_x = 1 + start_x + width as isize;
        let end_y = 1 + start_y + height as isize;
        let part = self.grid.get_part(start_x, start_y, end_x + 1, end_y);
        part.iter().enumerate().for_each(
            |v| {
                let color = Col(
                    match &v.1.feature {
                        Some(feature) => match feature {
                            CellFeature::Wall => Color::INDIGO
                        },
                        None => match v.1.cell_type {
                            CellType::Water  => Color::BLUE,
                            CellType::Ground => Color::ORANGE,
                            CellType::Grass  => Color::GREEN,
                            CellType::Stone  => Color::from_rgba(50,50,50,1.0),
                            CellType::Clicked => Color::PURPLE
                        }
                    }
                );
                let x = (v.1.x - (self.cam_x as isize - width as isize / 2)) * cell_size as isize;
                let y = (v.1.y - (self.cam_y as isize - height as isize / 2)) * cell_size as isize;
                window.draw(&Rectangle::new((x as f32 , y as f32), (cell_size as f32,cell_size as f32)), color);
            }
        );
        Ok(())
    }
    fn calc_start(cam : usize, line_size : usize) -> isize {
        let halved = line_size / 2;
        if cam < halved || cam == 1 {
            0
        } else {
            let calced = cam - halved;
            if calced <= 1 {
                0
            } else {
                calced as isize - 1
            }
        }
    }
}