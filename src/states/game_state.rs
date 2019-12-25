use crate::{
    funcs::{controls::check_multiple, math::sub_save},
    help_states::{Action, Grid, Mouse, Shop, WorldButtons},
    states::Screen,
    structs::{point::Point, CameraWork, FullContext, SaveableState, SimpleContext},
};
use quicksilver::{prelude::Key, Result};

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
    state: SaveableState,
    cam: CameraWork,
    clicked: Option<Point>,
    mode: ClickMode,
    selected: String,
    shop: Shop,
    updates: u64,
    open_window: OpenWindow,
    world_buttons: WorldButtons,
}
impl GameState {
    pub fn new(seed: u32, context: &mut SimpleContext) -> Self {
        let world_buttons = WorldButtons::new(context);
        world_buttons.set_state(false);
        let state = SaveableState::new((101, 81), seed, &context.assets.modules);
        let shop = Shop::new(context, &state.buyable_charachters);
        Self {
            state,
            clicked: None,
            shop,
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
            world_buttons,
        }
    }
    fn enable_gui_next_screen(&self, next_screen: OpenWindow) {
        match self.open_window {
            OpenWindow::Shop => self.shop.set_state(false),
            OpenWindow::Game => self.world_buttons.set_state(false),
        }
        match next_screen {
            OpenWindow::Shop => self.shop.set_state(true),
            OpenWindow::Game => self.world_buttons.set_state(true),
        }
    }
    fn set_enable_gui_next_screen(&mut self, next_screen: OpenWindow) {
        self.enable_gui_next_screen(next_screen);
        self.open_window = next_screen;
    }
}
impl Screen for GameState {
    fn update(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        let board = context.window.keyboard();
        if self.open_window != OpenWindow::Game && board[Key::Escape].is_down() {
            self.enable_gui_next_screen(OpenWindow::Game);
            self.open_window = OpenWindow::Game;
        }

        match self.open_window {
            OpenWindow::Shop => {
                let mut full_context = FullContext::new(&mut self.cam, context, &mut self.state);
                self.shop.update(&mut full_context);
                if let Some(next_screen) = full_context.get_next_screen() {
                    self.set_enable_gui_next_screen(next_screen);
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
                let mut full_context = FullContext::new(&mut self.cam, context, &mut self.state);
                let scroll = full_context.simple_context.mouse().wheel().y as isize;
                if scroll > full_context.cam_works.scroll as isize {
                    full_context.cam_works.scroll = 0
                } else {
                    let scroll = (full_context.cam_works.scroll as isize - scroll) as usize;
                    full_context.cam_works.scroll = scroll;
                };
                let action = self.world_buttons.update(&mut full_context);
                match action {
                    Action::Captured => {}
                    Action::SaveGame => {
                        quicksilver::saving::save("arena_keeper", "1", &self.state).unwrap()
                    }
                    Action::None => {
                        let mut mouse = Mouse {
                            clicked: &mut self.clicked,
                            mode: &mut self.mode,
                            selected: &self.selected,
                        };
                        mouse.update(&mut full_context);
                    }
                    Action::SwitchTool(tool, selected) => {
                        self.mode = tool;
                        self.selected = selected
                    }
                    Action::SwitchScreen(screen) => {
                        self.world_buttons.set_state(false);
                        self.set_enable_gui_next_screen(screen);
                    }
                }
                self.state
                    .bought_characters
                    .update(&mut self.state.grid, &context.assets.modules);
            }
        }
        Ok(None)
    }
    fn draw(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        self.updates += 1;
        let mut full_context = FullContext::new(&mut self.cam, context, &mut self.state);
        match self.open_window {
            OpenWindow::Shop => {
                self.shop.render(&mut full_context)?;
            }
            OpenWindow::Game => {
                //let mut grid = Grid::new(&self.state.grid);

                Grid::render(&mut full_context)?;

                let mut mouse = Mouse {
                    clicked: &mut self.clicked,
                    mode: &mut self.mode,
                    selected: &self.selected,
                };
                mouse.render(&mut full_context);
                drop(full_context);
                self.state.bought_characters.render(context, &self.cam);
                //self.world_buttons.draw(&mut full_context);
            }
        }
        Ok(None)
    }
}
