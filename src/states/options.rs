use crate::{
    mergui_wrapper::{drop_down, success_button, text, SimpleDropDownConfig},
    states::{MainMenu, Screen},
    structs::SimpleContext,
};
use mergui::{
    channels::{BasicClickable, Clickable, Dropdown},
    LayerId, Response,
};
use quicksilver::{geom::Rectangle, prelude::Vector, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SaveableOptions {
    pub resolution: Vector,
}
impl Default for SaveableOptions {
    fn default() -> Self {
        Self {
            resolution: (1280, 720).into(),
        }
    }
}
pub struct OptionsScreen {
    set_options: SaveableOptions,
    _layer: LayerId,
    resolution: Response<Dropdown<Vector>>,
    back: Response<BasicClickable>,
    set: Response<BasicClickable>,
    _res_text: Response<()>,
}

impl OptionsScreen {
    pub fn new(context: &mut SimpleContext) -> Result<Self> {
        let layer = context.gui.add_layer();
        let set_options = quicksilver::saving::load::<SaveableOptions>("arena_keeper", "options")
            .unwrap_or_else(|_| Default::default());

        let resolutions = vec![
            ((2560, 1440).into(), "2560x1440".into()),
            ((1920, 1080).into(), "1920x1080".into()),
            ((1366, 768).into(), "1366x768".into()),
            ((1280, 720).into(), "1280x720".into()),
            ((1920, 1200).into(), "1920x1200".into()),
            ((1680, 1050).into(), "1680x1050".into()),
            ((1440, 900).into(), "1440x900".into()),
            ((1280, 809).into(), "1280x800".into()),
            ((1024, 768).into(), "1024x768".into()),
            ((800, 600).into(), "800x600".into()),
            ((640, 480).into(), "640x480".into()),
        ];

        let selected = resolutions
            .iter()
            .enumerate()
            .find(|(_, v)| v.0 == set_options.resolution)
            .map(|(key, _)| key)
            .unwrap_or(0);

        let dropdown = drop_down::<Vector>(
            context.assets,
            SimpleDropDownConfig {
                location: Rectangle::new((450, 10), (200, 50)),
                values: resolutions,
                button_width: 50f32,
                selected,
            },
        )?;

        let resolution = context.gui.add_widget(dropdown, &layer).unwrap();
        let back = success_button(context.assets, Rectangle::new((10, 555), (55, 40)), "Back")?;
        let back = context.gui.add_widget(back, &layer).unwrap();
        let set = success_button(context.assets, Rectangle::new((704, 555), (55, 40)), "Set")?;
        let set = context.gui.add_widget(set, &layer).unwrap();
        let text = text(
            context.assets,
            Rectangle::new((50, 10), (130, 50)),
            "Resolution",
        )?;

        let text = context.gui.add_widget(text, &layer).unwrap();
        Ok(Self {
            _layer: layer,
            resolution,
            back,
            set,
            _res_text: text,
            set_options,
        })
    }
}

impl Screen for OptionsScreen {
    fn update(&mut self, context: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        let set = self.set.channel.has_clicked();
        let back = self.back.channel.has_clicked();
        if back {
            quicksilver::saving::save("arena_keeper", "options", &self.set_options)?;
            Ok(Some(Box::new(MainMenu::new(&context.assets, context.gui))))
        } else if set {
            if let Some(resolution) = self.resolution.channel.get_value() {
                context.window.set_size(resolution);
                self.set_options.resolution = resolution;
            }
            Ok(None)
        } else {
            Ok(None)
        }
    }
    fn draw(&mut self, _: &mut SimpleContext) -> Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }
}
