
use crate::help_states::characters::Characters;
use crate::structs::point::Point;
use crate::funcs::math::sub_save;
use crate::funcs::controls::check_multiple;
use quicksilver::prelude::Key;

use crate::structs::field::Field;
use crate::help_states::grid::Grid;
use crate::help_states::mouse::Mouse;
use crate::structs::camera_work::CameraWork;
use quicksilver::{
    Result,
    graphics::{Color},
    lifecycle::{State, Window},
};
pub struct GameState {
    grid : Field,
    cam : CameraWork,
    clicked : Option<(Point)>,
    characters : Characters
}
impl State for GameState {
     fn new() -> Result<Self> {
        Ok(Self {
            grid : Field::new(101,81,1032),
            clicked : None,
            characters : Characters::new(),
            cam : CameraWork {
                cam : (101/2 + 1,81/2 +1).into(),
                scroll : 100,
                width : 800,
                height : 600,

            }
        })
    }
    fn update(&mut self, window : &mut Window) -> Result<()> {
        let board = window.keyboard();

        if check_multiple(board,&[Key::Left,Key::A]) {
            self.cam.cam.x = sub_save(self.cam.cam.x,1);
        }
        if check_multiple(board,&[Key::Right,Key::D]) {
            self.cam.cam.x += 1;
        }
        if check_multiple(board,&[Key::Up,Key::W]) {
            self.cam.cam.y = sub_save(self.cam.cam.y,1);
        }
        if check_multiple(board,&[Key::Down,Key::S]) {
            self.cam.cam.y += 1;
        }
        let scroll = window.mouse().wheel().y as isize;
        if scroll > self.cam.scroll as isize {
            self.cam.scroll = 0
        } else {
            let scroll = (self.cam.scroll as isize - scroll) as usize;
            self.cam.scroll = scroll;
        };
        self.characters.update(&self.grid);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        Grid::new(&self.cam,&self.grid).render(window)?;
        Mouse::new(&self.cam,&mut self.clicked,&mut self.grid).render(window)?;
        self.characters.render(&self.cam, window);
        Ok(())
    }
}
impl GameState {
}