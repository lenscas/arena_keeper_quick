extern crate arena;
use arena::generated::assets::loaded::AssetManager;
use arena::generated::assets::to_load::load_all;
use arena::states::game_state::GameState;
use arena::funcs::math::sub_from_highest;

use quicksilver::{
    geom::Rectangle,
    input::{ButtonState, Key, MouseButton},
    lifecycle::Asset,
};
use std::rc::Rc;
use std::sync::Mutex;

use quicksilver::{
    geom::Vector,
    graphics::Color,
    lifecycle::{run, Settings, State, Window},
    Result,
};

pub struct DebugState {
    game_state: GameState,
    assets: Asset<AssetManager>,
    pause: bool,
    first_click: Option<Vector>,
    drawn_rectangles: Vec<(Rectangle,Color)>,
    current_color : Color
}
impl State for DebugState {
    fn new() -> Result<Self> {
        Ok(Self {
            game_state: GameState::new(rand::random()),
            assets: Asset::new(load_all()),
            pause: false,
            first_click: None,
            drawn_rectangles: Vec::new(),
            current_color : Color::from_rgba(0,0,0,1f32)
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let gamestate = &mut self.game_state;
        let test = Rc::new(Mutex::new(window));

        self.assets.execute_or(
            |asset| {
                let mut b = test.lock().unwrap();
                gamestate.draw(&mut b, asset)
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

        let gamestate = &mut self.game_state;
        self.assets.execute(|assets| gamestate.update(window,assets))
    }
}
impl DebugState {
    fn update_paused(&mut self, window: &mut Window) {
        let mouse = window.mouse();
        let pos = mouse.pos();
        let left_click = mouse[MouseButton::Left];
        if left_click == ButtonState::Pressed {
            if let Some(first_click) = self.first_click {
                let (left_corner,size) = calc_pos_and_width(&first_click,&pos);
                self.drawn_rectangles
                    .push((Rectangle::new(left_corner, size), self.current_color));
                self.first_click = None;
                use rand::random;
                self.current_color = Color::from_rgba(random(),random(),random(),1f32);
                println!("Point: {}\nsize: {}", left_corner,size);
            } else {
                println!("inside first click");
                self.first_click = Some(pos);
            }
        }
    }
    fn draw_paused(&mut self, window: &mut Window) {
        self.drawn_rectangles
            .iter()
            .for_each(|v| window.draw(&v.0, v.1 ));
        if let Some(first_click) = self.first_click {
            let (corner,size) = calc_pos_and_width(&first_click,&window.mouse().pos());
            window.draw(&Rectangle::new(corner,size), self.current_color);
        }
    }
}
fn calc_pos_and_width(pos1: &Vector, pos2: &Vector) -> (Vector,Vector) {
    let left_corner = Vector::new(
        if pos1.x < pos2.x { pos1.x } else { pos2.x },
        if pos1.y < pos2.y { pos1.y } else { pos2.y },
    );
    let width = sub_from_highest(pos1.x,pos2.x);
    let height = sub_from_highest(pos1.y,pos2.y);
    (left_corner,(width,height).into())
}
pub fn main() {
    run::<DebugState>("Arena (debug)", Vector::new(800, 600), Settings::default());
}