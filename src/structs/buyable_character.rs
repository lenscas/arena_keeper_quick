use crate::generated::assets::loaded::Images;
use crate::generated::species::images::get_random_image;
use crate::generated::species::{
    names::get_random_name,
    species::Species
};
use rand::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BuyableCharacter {
    name: String,
    walk_speed: usize,
    pub cost: u32,
    species : Species,
    image : Images
}
impl Default for BuyableCharacter {
    fn default() -> Self {
        Self::new()
    }
}

impl BuyableCharacter {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let species : Species = rand::random();
        let name = get_random_name(species);
        let image = get_random_image(species);
        Self {
            name,
            walk_speed: rng.gen_range(1, 8),
            cost: rng.gen_range(10, 20),
            species,
            image
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_speed(&self) -> usize {
        self.walk_speed
    }
    pub fn get_species(&self) -> Species {
        self.species
    }
    pub fn get_image(&self) -> Images {
        self.image
    }
}
