use arena::{assets::to_load::load_all, funcs::math::sub_from_highest, states::StateManager};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
use std::{
    rc::Rc,
    sync::{mpsc, mpsc::Receiver, Mutex},
    thread,
    thread::JoinHandle,
};

pub struct DebugState<'a> {
    assets: Asset<StateManager<'a>>,
    pause: bool,
    first_click: Option<Vector>,
    drawn_rectangles: Vec<(Rectangle, Color)>,
    current_color: Color,
    _command_reader: JoinHandle<()>,
    command_getter: Receiver<String>,
}
impl State for DebugState<'static> {
    fn new() -> Result<Self> {
        let (tx, rx) = mpsc::channel();
        use std::io::stdin;
        let handle = thread::spawn(move || loop {
            let mut s = String::new();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
            tx.send(s).unwrap();
        });
        Ok(Self {
            assets: Asset::new(load_all().and_then(|v| StateManager::new(v))),
            pause: false,
            first_click: None,
            drawn_rectangles: Vec::new(),
            current_color: Color::from_rgba(0, 0, 0, 1f32),
            command_getter: rx,
            _command_reader: handle,
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let test = Rc::new(Mutex::new(window));

        self.assets.execute_or(
            |state| {
                let mut b = test.lock().unwrap();
                state.draw(&mut b)
            },
            || {
                let mut b = test.lock().unwrap();
                b.clear(Color::RED)
            },
        )?;
        if self.pause {
            self.draw_paused(&mut test.lock().unwrap());
        }
        Ok(())
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        let keyboard = window.keyboard();

        if self.pause && keyboard[Key::P] == ButtonState::Pressed {
            self.pause = false;
        }

        if keyboard[Key::LControl].is_down() && keyboard[Key::P] == ButtonState::Pressed {
            self.pause = true;
        }

        if self.pause {
            self.update_paused(window);
            return Ok(());
        }
        self.assets.execute(|state| state.update(window))
    }
}
impl DebugState<'static> {
    fn update_paused(&mut self, window: &mut Window) {
        let mouse = window.mouse();
        let pos = mouse.pos();
        let left_click = mouse[MouseButton::Left];
        if left_click == ButtonState::Pressed {
            if let Some(first_click) = self.first_click {
                let (left_corner, size) = calc_pos_and_width(&first_click, &pos);
                self.drawn_rectangles
                    .push((Rectangle::new(left_corner, size), self.current_color));
                self.first_click = None;
                use rand::random;
                self.current_color = Color::from_rgba(random(), random(), random(), 1f32);
            } else {
                self.first_click = Some(pos);
            }
        }
        loop {
            let v = self.command_getter.try_recv().ok();
            let v = v
                .as_ref()
                .map(|s| s.split_ascii_whitespace().clone().collect::<Vec<_>>());
            let res = if let Some(x) = v { x } else { break };
            let res: std::result::Result<(), String> = match res[0] {
                "read" => {
                    if let Some(square) = res.get(1).map(|v| {
                        v.parse::<usize>()
                            .ok()
                            .map(|v| self.drawn_rectangles.get_mut(v))
                    }) {
                        if let Some(square) = square {
                            if let Some(_) = square {
                                std::result::Result::Ok(())
                            } else {
                                std::result::Result::Err(String::from("Couldn't parse output"))
                            }
                        } else {
                            std::result::Result::Err(String::from("Couldn't parse output"))
                        }
                    } else {
                        std::result::Result::Err(String::from("Couldn't parse output"))
                    }
                }
                x => Err(format!("{} is not a valid command", x)),
            };
            match res {
                Ok(_) => {}
                Err(x) => println!("{}", x),
            }
        }
    }
    fn draw_paused(&mut self, window: &mut Window) {
        self.drawn_rectangles
            .iter()
            .for_each(|v| window.draw(&v.0, v.1));
        if let Some(first_click) = self.first_click {
            let (corner, size) = calc_pos_and_width(&first_click, &window.mouse().pos());
            window.draw(&Rectangle::new(corner, size), self.current_color);
        }
    }
}
fn calc_pos_and_width(pos1: &Vector, pos2: &Vector) -> (Vector, Vector) {
    let left_corner = Vector::new(
        if pos1.x < pos2.x { pos1.x } else { pos2.x },
        if pos1.y < pos2.y { pos1.y } else { pos2.y },
    );
    let width = sub_from_highest(pos1.x, pos2.x);
    let height = sub_from_highest(pos1.y, pos2.y);
    (left_corner, (width, height).into())
}
pub fn main() {
    run::<DebugState>("Arena (debug)", Vector::new(800, 600), Settings::default());
}
