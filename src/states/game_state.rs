use crate::{
    funcs::{controls::check_multiple, math::sub_save},
    help_states::{Characters, Grid, Mouse},
    structs::{grid::Field, point::Point, CameraWork},
};
use quicksilver::{
    graphics::Color,
    lifecycle::{State, Window},
    prelude::Key,
    Result,
};
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
}
impl State for GameState {
    fn new() -> Result<Self> {
        Ok(Self {
            grid: Field::new(101, 81, 1032),
            clicked: None,
            characters: Characters::new(),
            cam: CameraWork {
                cam: (101 / 2 + 1, 81 / 2 + 1).into(),
                scroll: 100,
                width: 800,
                height: 600,
            },
            selected: ClickMode::Bed,
        })
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        let board = window.keyboard();

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
        let scroll = window.mouse().wheel().y as isize;
        if scroll > self.cam.scroll as isize {
            self.cam.scroll = 0
        } else {
            let scroll = (self.cam.scroll as isize - scroll) as usize;
            self.cam.scroll = scroll;
        };
        self.characters.update(&mut self.grid);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        Grid::new(&self.cam, &self.grid).render(window)?;
        Mouse {
            cam: &self.cam,
            clicked: &mut self.clicked,
            grid: &mut self.grid,
            selected: &mut self.selected,
        }
        .render(window)?;
        self.characters.render(&self.cam, window);
        Ok(())
    }
}
impl GameState {}
