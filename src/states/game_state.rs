use crate::{
    funcs::{controls::check_multiple, math::sub_save},
    help_states::{Action, Characters, Grid, Mouse, Shop, WorldButtons},
    states::Screen,
    structs::{grid::Field, point::Point, CameraWork, FullContext, SimpleContext},
};
use quicksilver::{
    geom::Line,
    graphics::Color,
    prelude::{Col, Key},
    Result,
};

#[derive(PartialEq, Clone, Copy)]
pub enum OpenWindow {
    Shop,
    Game,
}
#[derive(PartialEq, Clone, Copy)]
pub enum ClickMode {
    Single,
    Line,
}
pub struct GameState {
    grid: Field,
    cam: CameraWork,
    clicked: Option<(Point)>,
    characters: Characters,
    mode: ClickMode,
    selected: String,
    shop: Shop,
    updates: u64,
    open_window: OpenWindow,
    world_buttons: WorldButtons,
}
impl GameState {
    pub fn new(seed: u32, context: &mut SimpleContext) -> Self {
        Self {
            grid: Field::new(101, 81, seed, &context.assets.modules),
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
            mode: ClickMode::Single,
            selected: context
                .assets
                .modules
                .all_features
                .keys()
                .next()
                .expect("No tile features available")
                .into(),
            world_buttons: WorldButtons::new(context.assets),
        }
    }
}
impl Screen for GameState {
    fn update(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        let board = context.keyboard();
        if self.open_window != OpenWindow::Game && board[Key::Escape].is_down() {
            self.open_window = OpenWindow::Game;
        }

        match self.open_window {
            OpenWindow::Shop => {
                let mut full_context = FullContext::new(&mut self.cam, context);
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
                let mut full_context = FullContext::new(&mut self.cam, context);
                let scroll = full_context.simple_context.mouse().wheel().y as isize;
                if scroll > full_context.cam_works.scroll as isize {
                    full_context.cam_works.scroll = 0
                } else {
                    let scroll = (full_context.cam_works.scroll as isize - scroll) as usize;
                    full_context.cam_works.scroll = scroll;
                };
                let action = self.world_buttons.update(&mut full_context);
                match action {
                    Action::None => {
                        let mut mouse = Mouse {
                            clicked: &mut self.clicked,
                            grid: &mut self.grid,
                            mode: &mut self.mode,
                            selected: &self.selected,
                        };
                        mouse.update(&mut full_context);
                    }
                    Action::SwitchTool(tool, selected) => {
                        self.mode = tool;
                        self.selected = selected
                    }
                    Action::SwitchScreen(screen) => self.open_window = screen,
                }
                self.characters
                    .update(&mut self.grid, &context.assets.modules);
            }
        }
        Ok(None)
    }
    fn draw(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        self.updates += 1;
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
                    mode: &mut self.mode,
                    selected: &self.selected,
                };
                mouse.render(&mut full_context);
                self.characters.render(&mut full_context);
                self.world_buttons.draw(&mut full_context);
            }
        }

        let line = Line::new((0, 550), (800, 550)).with_thickness(5);
        full_context.simple_context.draw(&line, Col(Color::BLACK));
        Ok(None)
    }
}
