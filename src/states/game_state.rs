use crate::structs::SimpleContext;
use crate::states::Screen;
use crate::{
    funcs::{controls::check_multiple, math::sub_save},
    help_states::{Characters, Grid, Mouse, Shop},
    structs::{grid::Field, point::Point, CameraWork, FullContext},
};
use quicksilver::prelude::Background::Col;
use quicksilver::{geom::Line, graphics::Color, prelude::Key, Result};
#[derive(PartialEq, Clone, Copy)]
pub enum OpenWindow {
    Shop,
    Game,
}
pub enum ClickMode {
    Wall,
    Bed,
}
pub struct GameState {
    grid: Field,
    cam: CameraWork,
    clicked: Option<(Point)>,
    characters: Characters,
    selected: ClickMode,
    shop: Shop,
    updates: u64,
    open_window: OpenWindow,
}
impl GameState {
    pub fn new(seed: u32, context : &mut SimpleContext) -> Self {
        Self {
            grid: Field::new(101, 81, seed),
            clicked: None,
            characters: Characters::new(),
            shop: Shop::new(context),
            cam: CameraWork {
                cam: (101 / 2 + 1, 81 / 2 + 1).into(),
                scroll: 100,
                width: 800,
                height: 600,
            },
            updates: 0,
            open_window: OpenWindow::Shop,
            selected: ClickMode::Bed,
        }
    }
}
impl Screen for GameState {
    fn update(&mut self, context : &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        let board = context.keyboard();
        if self.open_window != OpenWindow::Game && board[Key::Escape].is_down() {
            self.open_window = OpenWindow::Game;
        }

        match self.open_window {
            OpenWindow::Shop => {
                let mut full_context = FullContext::new( &mut self.cam, context);
                self.shop.update(&mut full_context, &mut self.characters);
                if let Some(next_screen) = full_context.get_next_screen() {
                    self.open_window = next_screen;
                }
            }
            OpenWindow::Game => {
                if check_multiple(board, &[Key::Left, Key::A]) {
                    self.cam.cam.x = sub_save(self.cam.cam.x, 1);
                }
                if check_multiple(board, &[Key::Right, Key::D]) {
                    self.cam.cam.x += 1;
                }
                if check_multiple(board, &[Key::Up, Key::W]) {
                    self.cam.cam.y = sub_save(self.cam.cam.y, 1);
                }
                if check_multiple(board, &[Key::Down, Key::S]) {
                    self.cam.cam.y += 1;
                }
                let mut full_context = FullContext::new( &mut self.cam, context);
                let scroll = full_context.simple_context.mouse().wheel().y as isize;
                if scroll > full_context.cam_works.scroll as isize {
                    full_context.cam_works.scroll = 0
                } else {
                    let scroll = (full_context.cam_works.scroll as isize - scroll) as usize;
                    full_context.cam_works.scroll = scroll;
                };
                self.characters.update(&mut self.grid);
                let mut mouse = Mouse {
                    clicked: &mut self.clicked,
                    grid: &mut self.grid,
                    selected: &mut self.selected,
                };
                mouse.update(&mut full_context);
            }
        }
        Ok(None)
    }
    fn draw(&mut self, context : &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        self.updates += 1;
        /*
        if self.updates == 1 {
            self.shop.first_render(assets);
        }
        */
        let mut full_context = FullContext::new(&mut self.cam, context);
        match self.open_window {
            OpenWindow::Shop => {
                self.shop.render(&mut full_context)?;
            }
            OpenWindow::Game => {
                let mut grid = Grid::new(&self.grid);

                grid.render(&mut full_context)?;
                let mut mouse = Mouse {
                    clicked: &mut self.clicked,
                    grid: &mut self.grid,
                    selected: &mut self.selected,
                };
                mouse.render(&mut full_context);
                self.characters.render(&mut full_context);
            }
        }

        let line = Line::new((0, 550), (800, 550)).with_thickness(5);
        full_context.simple_context.draw(&line, Col(Color::BLACK));
        Ok(None)
    }
}
