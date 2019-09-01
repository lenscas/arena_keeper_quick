use crate::{
    help_states::BuyableInfo,
    generated::assets::loaded::{AssetManager, Fonts},
    structs::{
            gui_2::{
            button::{
                Builder,
                State
            },
            Image,
            Combined,
            ButtonBackground,
            Interaction
        },
        FullContext,
    },
    help_states::characters::Characters, structs::BuyableCharacter,
    states::game_state::OpenWindow
};
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, FontStyle, Font},
    Result,
};

#[derive(Default)]
pub struct Shop {
    money: u32,
    assets: Vec<(State, BuyableCharacter)>,
    go_to_game_button : Option<Combined<State,ButtonBackground>>,
    selected : Option<(usize, BuyableInfo)>,
    show_money: Option<Image>,
    up_button : Option<Combined<State,ButtonBackground>>
}

impl Shop {
    pub fn new() -> Self {
        Self {
            money: 100,
            assets: Vec::new(),
            go_to_game_button : None,
            selected : None,
            show_money: None,
            up_button:None
        }
    }
    pub fn first_render(&mut self, assets: &AssetManager) {
        let chars = vec![
            BuyableCharacter::new(),
            BuyableCharacter::new(),
            BuyableCharacter::new(),
        ];
        self.assets = chars
            .iter()
            .cloned()
            .enumerate()
            .map(|(count, v)| {
                let name = v.get_name();
                let builder = Builder::new_single_text(name);
                let style = FontStyle::new(50.1, Color::BLACK);
                let button = builder
                    .to_state(
                        Fonts::Font,
                        style,
                        assets,
                        Rectangle::new((10, 90 + (count as i32 * 50)), (90, 50)),
                    )
                    .unwrap();
                (button, v)
            })
            .collect();
        self.up_button = Some(ButtonBackground::new_success(assets,Rectangle::new((10,40),(90,50)),"Up".to_string()));
        self.go_to_game_button = Some(ButtonBackground::new_success(assets,Rectangle::new((10,555), (55, 40)),"World".to_string()));
        self.update_show_money(assets);
        
    }
    fn update_show_money(&mut self, assets : &AssetManager) {
        self.show_money = {
            let img = Image::new(
                Font::render(
                    assets.font(&Fonts::Font),
                    &self.money.to_string(),
                    &FontStyle::new(50.1,Color::BLACK)
                ).unwrap(),
                Rectangle::new((10,10),(30,20))
            );
            Some(img)
        }
    }
    pub fn update(&mut self, context : &mut FullContext,characters_state: &mut Characters,) {
        if let Some(selected) = &mut self.selected {
            if selected.1.did_buy(context) {
                let bought = self.assets.remove(selected.0).1;
                if bought.cost < self.money {
                    self.money -= bought.cost;
                    self.update_show_money(context.get_assets());
                    characters_state.add_character(bought);
                    self.selected = None;
                    
                }
            }
        }
        let selected = self.assets
            .iter_mut()
            .enumerate()
            .map(|(count, button)| (count,context.get_interaction(&mut button.0)))
            .filter(|v| v.1 == Interaction::Clicked)
            .collect::<Vec<_>>()
            .iter()
            .find(|v| v.1 == Interaction::Clicked)
            .map(|v| (v.0, BuyableInfo::new(&self.assets[v.0].1, context)));
        if let Some(selected) = selected {
            self.selected = Some(selected);
        }
        if self.go_to_game_button.iter_mut().map(|v| context.get_interaction(v)).any(|v| v == Interaction::Clicked) {
            context.set_next_screen(Some(OpenWindow::Game));
        }
    }
    pub fn render(
        &mut self,
        context: &mut FullContext,
    ) -> Result<()> {
        self.assets
            .iter()
            .cloned()
            .for_each(|(button,_)| {
                context.push_widget(button);
            });
        //context.push_widget(self.up_button.clone().unwrap());
        context.push_widget(self.show_money.clone().unwrap());
        context.push_widget(self.go_to_game_button.clone().unwrap());
        if let Some(info) = &mut self.selected {
            info.1.draw(context);
        }
        Ok(())
    }
}
