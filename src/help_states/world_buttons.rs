use crate::{
    mergui_wrapper::success_button,
    states::{ClickMode, OpenWindow},
    structs::{FullContext, SimpleContext},
};
use std::collections::HashMap;
use std::marker::PhantomData;

use mergui::{
    channels::{BasicClickable, Clickable, ConcealerManagerReturn},
    widgets::{ConcealerConfig, ConcealerManagerConfig},
    LayerId, Response,
};
use quicksilver::geom::Rectangle;

pub struct WorldButtons {
    item_buttons: Response<ConcealerManagerReturn<String, BasicClickable>>,
    actions: HashMap<String, Action>,
    shop_button: Response<BasicClickable>,
    layer: LayerId,
}
impl WorldButtons {
    pub fn new(context: &mut SimpleContext) -> Self {
        let layer = context.gui.add_layer();
        let mut actions = HashMap::new();

        let all_features = &context.assets.modules.all_features;
        let assets = &context.assets;
        let button_size_x = 100;
        let button_size = (button_size_x, 55);
        let button_y = 547;
        let start_first_button = 10;
        let button_marging = 5;
        let buttons = assets
            .modules
            .all_categories
            .iter()
            .map(|(category, features)| {
                (
                    category,
                    features
                        .iter()
                        .map(|val| (val, all_features.get(val).unwrap()))
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
                                    assets,
                                    Rectangle::new(
                                        (
                                            start_first_button
                                                + (index as i32 * button_size_x)
                                                + (index as i32 * button_marging),
                                            495,
                                        ),
                                        button_size.clone(),
                                    ),
                                    &button_name,
                                )
                                .unwrap(),
                            )
                        })
                        .collect(),
                )
            })
            .enumerate()
            .map(|(key, buttons)| ConcealerConfig {
                button: success_button(
                    assets,
                    Rectangle::new(
                        (
                            start_first_button
                                + (1 + key) as i32 * button_size_x
                                + (key as i32 + 1) * button_marging,
                            button_y,
                        ),
                        button_size.clone(),
                    ),
                    buttons.0,
                )
                .unwrap(),
                hidden_widgets: buttons.1,
                to_result: PhantomData,
                to_widget: PhantomData,
            })
            .collect::<Vec<_>>();
        drop(assets);
        drop(all_features);
        let item_buttons = context
            .gui
            .add_widget(
                ConcealerManagerConfig {
                    concealers: buttons,
                },
                &layer,
            )
            .unwrap();
        Self {
            item_buttons,
            actions,
            shop_button: context
                .gui
                .add_widget(
                    success_button(
                        context.assets,
                        Rectangle::new((start_first_button, button_y), button_size.clone()),
                        "Shop",
                    )
                    .unwrap(),
                    &layer,
                )
                .unwrap(),
            layer,
        }
    }
    pub fn update(&mut self, _: &mut FullContext) -> Action {
        let has_clicked_shop = self.shop_button.channel.has_clicked();
        let has_clicked_actions = self
            .item_buttons
            .channel
            .buttons_iter_mut()
            .map(|button| button.has_clicked())
            .fold(false, |cur, now| cur || now);
        if has_clicked_shop {
            Action::SwitchScreen(OpenWindow::Shop)
        } else if has_clicked_actions {
            Action::Captured
        } else {
            let actions = &self.actions;
            self.item_buttons
                .channel
                .iter_mut()
                .map(|v| {
                    v.iter_mut()
                        .map(|(name, widget)| (name, widget.has_clicked()))
                        .collect::<Vec<_>>()
                        .into_iter()
                        .find(|(_, has_clicked)| *has_clicked)
                        .and_then(|(name, _)| actions.get(name).cloned())
                        .unwrap_or(Action::None)
                })
                .fold(Action::None, |cur, now| match (cur, now) {
                    (Action::None, x) => x,
                    (x, _) => x,
                })
        }
    }
    pub fn set_state(&self, state: bool) {
        self.layer.set_is_active(state)
    }
}
#[derive(Clone)]
pub enum Action {
    None,
    Captured,
    SwitchScreen(OpenWindow),
    SwitchTool(ClickMode, String),
}
