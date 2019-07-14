
use crate::help_states::Shop;
use crate::{
    help_states::{
        Characters,
        Grid,
        Mouse
    },
    structs::{
        point::Point,
        grid::Field,
        CameraWork
    },
    funcs::{
        math::sub_save,
        controls::check_multiple
    }
};
use quicksilver::{
    Result,
    prelude::Key,
    graphics::{Color},
    lifecycle::{State, Window},
};
#[derive(PartialEq)]
enum OpenWindow {
    Shop,
    Game
}
pub struct GameState {
    grid : Field,
    cam : CameraWork,
    clicked : Option<(Point)>,
    characters : Characters,
    shop : Shop,
    open_window : OpenWindow
}
impl State for GameState {
     fn new() -> Result<Self> {
        Ok(Self {
            grid : Field::new(101,81,1032),
            clicked : None,
            characters : Characters::new(),
            shop : Shop::new(),
            cam : CameraWork {
                cam : (101/2 + 1,81/2 +1).into(),
                scroll : 100,
                width : 800,
                height : 600,

            },
            open_window : OpenWindow::Shop
        })
    }
    fn update(&mut self, window : &mut Window) -> Result<()> {
        let board = window.keyboard();
        if self.open_window != OpenWindow::Game && board[Key::Escape].is_down() {
            self.open_window = OpenWindow::Game;
        }
        match self.open_window {
            OpenWindow::Shop => {},
            OpenWindow::Game => {
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
            }
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        match self.open_window {
            OpenWindow::Shop => {self.shop.render(window, &mut self.characters)? },
            OpenWindow::Game => {
                Grid::new(&self.cam,&self.grid).render(window)?;
                Mouse::new(&self.cam,&mut self.clicked,&mut self.grid).render(window)?;
                self.characters.render(&self.cam, window);

            }
        }
        Ok(())
    }
}
impl GameState {
}