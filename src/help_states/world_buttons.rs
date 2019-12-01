use crate::{
    states::{ClickMode, OpenWindow},
    structs::{gui_2::success_button, FullContext, SimpleContext},
};
use std::collections::HashMap;
use std::marker::PhantomData;

use mergui::{
    channels::{BasicClickable, Clickable, Concealer, ConcealerReturn},
    widgets::ConcealerConfig,
    LayerId, Response,
};
use quicksilver::geom::Rectangle;

pub struct WorldButtons {
    item_buttons: Response<ConcealerReturn<String, BasicClickable>>,
    actions: HashMap<String, Action>,
    shop_button: Response<BasicClickable>,
    layer: LayerId,
}
impl WorldButtons {
    pub fn new(context: &mut SimpleContext) -> Self {
        let layer = context.gui.add_layer();
        //let mut buttons = vec![(Action::SwitchScreen(OpenWindow::Shop), String::from("Shop"))];
        let mut actions = HashMap::new();
        let buttons = context
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
            })
            .map(|(action, name)| {
                actions.insert(name.clone(), action);
                name
            })
            .enumerate()
            .map(|(index, button_name)| {
                (
                    button_name.clone(),
                    success_button(
                        context.assets,
                        Rectangle::new((10 + (index as i32 * 56), 500), (55, 40)),
                        &button_name,
                    )
                    .unwrap(),
                )
            })
            .collect();
        let config = ConcealerConfig {
            button: success_button(
                context.assets,
                Rectangle::new((10 + (1 as i32 * 56), 555), (55, 40)),
                "Actions",
            )
            .unwrap(),
            hidden_widgets: buttons,
            to_result: PhantomData,
            to_widget: PhantomData,
        };
        Self {
            item_buttons: context.gui.add_widget(config, &layer).unwrap(),
            actions,
            shop_button: context
                .gui
                .add_widget(
                    success_button(context.assets, Rectangle::new((10, 555), (55, 40)), "Shop")
                        .unwrap(),
                    &layer,
                )
                .unwrap(),
            layer,
        }
    }
    pub fn update(&mut self, _: &mut FullContext) -> Action {
        let has_clicked_shop = self.shop_button.channel.has_clicked();
        let has_clicked_actions = self.item_buttons.channel.has_clicked();
        if has_clicked_shop {
            Action::SwitchScreen(OpenWindow::Shop)
        } else if has_clicked_actions {
            Action::Captured
        } else {
            let actions = &self.actions;
            self.item_buttons
                .channel
                .iter_mut()
                .map(|(name, widget)| (name, widget.has_clicked()))
                .collect::<Vec<_>>()
                .into_iter()
                .find(|(_, has_clicked)| *has_clicked)
                .and_then(|(name, _)| actions.get(name).cloned())
                .unwrap_or(Action::None)
        }
    }
    pub fn set_state(&self, state: bool) {
        self.layer.set_is_active(state)
    }
    pub fn draw(&self, _: &mut FullContext) {}
}
#[derive(Clone)]
pub enum Action {
    None,
    Captured,
    SwitchScreen(OpenWindow),
    SwitchTool(ClickMode, String),
}
