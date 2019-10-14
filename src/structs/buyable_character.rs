use crate::{
    assets::loaded::{AssetManager, Images},
    modules::structs::SpeciesType,
};
use rand::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BuyableCharacter {
    name: String,
    walk_speed: usize,
    pub cost: u32,
    species: SpeciesType,
    image: Images,
}

impl BuyableCharacter {
    pub fn new(assets: &AssetManager) -> Self {
        let mut rng = rand::thread_rng();
        let species = assets.modules.get_random_species();
        let name = assets.modules.get_random_name_for_species(&species);
        let image = assets.modules.get_random_image_for_species(&species);
        let walk_speed = assets.modules.get_random_base_speed(&species);
        Self {
            name,
            walk_speed,
            cost: rng.gen_range(10, 20),
            species,
            image,
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_speed(&self) -> usize {
        self.walk_speed
    }
    pub fn get_species(&self) -> SpeciesType {
        self.species.clone()
    }
    pub fn get_image(&self) -> Images {
        self.image.clone()
    }
}
