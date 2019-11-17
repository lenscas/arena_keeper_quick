use crate::{
    assets::loaded::AssetManager,
    states::{ClickMode, OpenWindow},
    structs::{
        gui_2::{button::State, ButtonBackground, Combined, Interaction},
        FullContext,
    },
};

use quicksilver::geom::Rectangle;

pub struct WorldButtons {
    buttons: Vec<(Action, Combined<State, ButtonBackground>)>,
}
impl WorldButtons {
    pub fn new(assets: &AssetManager) -> Self {
        let mut buttons = vec![(Action::SwitchScreen(OpenWindow::Shop), String::from("Shop"))];
        buttons.extend(assets.modules.all_features.iter().map(|(key, val)| {
            (
                Action::SwitchTool(
                    if val.is_drawable {
                        ClickMode::Line
                    } else {
                        ClickMode::Single
                    },
                    key.clone(),
                ),
                val.name.clone(),
            )
        }));
        let buttons = buttons
            .drain(0..buttons.len())
            .enumerate()
            .map(|v| {
                (
                    (v.1).0,
                    ButtonBackground::new_success(
                        assets,
                        Rectangle::new((10 + (v.0 as i32 * 56), 555), (55, 40)),
                        (v.1).1,
                    ),
                )
            })
            .collect();
        Self { buttons }
    }
    pub fn update(&mut self, context: &mut FullContext) -> Action {
        self.buttons
            .iter_mut()
            .map(|v| (&v.0, context.simple_context.get_interaction(&mut v.1)))
            .collect::<Vec<_>>()
            .iter()
            .find(|v| v.1 == Interaction::Clicked)
            .map(|v| v.0.clone())
            .unwrap_or(Action::None)
    }
    pub fn draw(&self, context: &mut FullContext) {
        self.buttons
            .iter()
            .map(|v| v.1.clone())
            .for_each(|v| context.simple_context.push_widget(v));
    }
}
#[derive(Clone)]
pub enum Action {
    None,
    SwitchScreen(OpenWindow),
    SwitchTool(ClickMode, String),
}
