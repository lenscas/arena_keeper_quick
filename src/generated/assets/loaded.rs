use crate::modules::structs::ModulesContainer;
use quicksilver::graphics::Font;
use quicksilver::graphics::Image;
use std::collections::HashMap;

pub type Images = String;
/*
pub enum Images {
    GeneratedHumanBarbarianIdle01,
    GeneratedHumanBarbarianIdle02,
    GeneratedHumanBarbarianIdle03,
    GeneratedHumanBarbarianIdle04,
    GeneratedMerfolkLizardIdle01,
    GeneratedMerfolkLizardIdle02,
    GeneratedMerfolkLizardIdle03,
    GeneratedMerfolkLizardIdle04,
    GeneratedTilesGrass,
    GeneratedTilesSand,
    GeneratedTilesStone,
    GeneratedTilesWater,
    TestButton
}
*/
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Fonts {
    Font,
}

#[derive(Default)]
pub struct AssetManager {
    images: HashMap<Images, Image>,
    fonts: HashMap<Fonts, Font>,
    pub modules: ModulesContainer,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            fonts: HashMap::new(),
            modules: Default::default(),
        }
    }
    pub fn insert_image(&mut self, at: Images, image: Image) {
        self.images.insert(at, image);
    }
    pub fn image(&self, at: &str) -> &Image {
        match self.images.get(at) {
            Some(x) => x,
            None => self.modules.get_image_by_name(at),
        }
    }
    pub fn insert_font(&mut self, at: Fonts, font: Font) {
        self.fonts.insert(at, font);
    }
    pub fn font(&self, at: &Fonts) -> &Font {
        self.fonts.get(at).unwrap()
    }
}
