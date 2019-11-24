use crate::{
    states::{ClickMode, OpenWindow},
    structs::{gui_2::success_button, FullContext, SimpleContext},
};

use mergui::{
    channels::{BasicClickable, Clickable},
    Context, Response,
};
use quicksilver::geom::Rectangle;

pub struct WorldButtons {
    buttons: Vec<(Action, Response<BasicClickable>)>,
    layer: u64,
}
impl WorldButtons {
    pub fn new(context: &mut SimpleContext) -> Self {
        let layer = context.gui.add_layer();
        let mut buttons = vec![(Action::SwitchScreen(OpenWindow::Shop), String::from("Shop"))];
        buttons.extend(
            context
                .assets
                .modules
                .all_features
                .iter()
                .map(|(key, val)| {
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
                }),
        );
        let buttons = buttons
            .into_iter()
            .enumerate()
            .map(|v| {
                (
                    (v.1).0,
                    context
                        .gui
                        .add_widget(
                            success_button(
                                context.assets,
                                Rectangle::new((10 + (v.0 as i32 * 56), 555), (55, 40)),
                                &(v.1).1,
                            )
                            .unwrap(),
                            layer,
                        )
                        .unwrap(),
                )
            })
            .collect();
        Self { buttons, layer }
    }
    pub fn update(&mut self, _: &mut FullContext) -> Action {
        let res = self
            .buttons
            .iter_mut()
            .map(|v| (&v.0, v.1.channel.has_clicked()))
            .find(|v| v.1)
            .map(|v| v.0.clone())
            .unwrap_or(Action::None);
        res
    }
    pub fn set_state<'a>(&self, context: &mut Context<'a>, state: bool) {
        context.set_layer_state(self.layer, state)
    }
    pub fn draw(&self, _: &mut FullContext) {}
}
#[derive(Clone)]
pub enum Action {
    None,
    SwitchScreen(OpenWindow),
    SwitchTool(ClickMode, String),
}
