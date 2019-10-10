use rand::seq::{IteratorRandom, SliceRandom};
use std::{collections::HashMap, path::Path};

use quicksilver::graphics::Image;

#[derive(Default)]
pub struct Module {
    species: HashMap<String, SpeciesConf>,
    images: HashMap<String, Image>,
    tiles: Option<TilesConf>,
}
impl Module {
    pub fn new() -> Self {
        Self {
            species: HashMap::new(),
            images: HashMap::new(),
            tiles: None,
        }
    }
    pub fn add_image(&mut self, path: &Path, img: Image) {
        let path = path.to_path_buf().into_os_string().into_string();
        match path {
            Ok(path) => {
                self.images.insert(path, img);
            }
            Err(err) => println!("Could not convert path to String : {:?}", err),
        }
    }
    pub fn set_species(&mut self, name: String, species: SpeciesConf) {
        self.species.insert(name, species);
    }
    pub fn set_tiles(&mut self, tiles: TilesConf) {
        self.tiles = Some(tiles);
    }
}

#[derive(serde::Deserialize)]
pub struct SpeciesConf {
    speeds: HashMap<String, f32>,
    pub name: String,
    possibleNames: Vec<String>,
    images: Vec<ImageName>,
}

#[derive(serde::Deserialize)]
pub struct Tile {
    end: f64,
    image: String,
}
#[derive(serde::Deserialize)]
pub struct TilesConf {
    generateChances: HashMap<String, Tile>,
}

pub type SpeciesType = String;
pub type ImageName = String;
pub type TileType = String;

#[derive(Default)]
pub struct ModulesContainer {
    //modules : Vec<Module>,
    all_species: HashMap<SpeciesType, SpeciesConf>,
    all_tiles: HashMap<String, Tile>,
    all_images: HashMap<ImageName, Image>,
}
impl ModulesContainer {
    pub fn add_module(&mut self, module: Module) {
        self.all_species.extend(module.species);
        if let Some(tiles) = module.tiles {
            self.all_tiles.extend(tiles.generateChances)
        }
        self.all_images.extend(module.images);
    }
    pub fn get_random_image_for_species(&self, species: &str) -> ImageName {
        let mut rng = rand::thread_rng();
        self.all_species
            .get(species)
            .unwrap()
            .images
            .choose(&mut rng)
            .unwrap()
            .clone()
    }
    pub fn f64_to_tile(&self, num: f64) -> Option<String> {
        let found: Option<(&str, &Tile)> = None;
        let num = num + 1.0;
        self.all_tiles
            .iter()
            .fold(found, |found, (name, tile)| {
                if tile.end > num {
                    if found.is_none() || found.unwrap().1.end - num > tile.end - num {
                        Some((name, tile))
                    } else {
                        found
                    }
                } else {
                    found
                }
            })
            .map(|v| v.0.into())
    }
    pub fn get_random_name_for_species(&self, species: &str) -> String {
        let mut rng = rand::thread_rng();
        self.all_species
            .get(species)
            .unwrap()
            .possibleNames
            .choose(&mut rng)
            .unwrap()
            .clone()
    }
    pub fn get_image_by_name(&self, name: &str) -> &Image {
        self.all_images.get(name).unwrap()
    }
    pub fn get_random_species(&self) -> SpeciesType {
        let mut rng = rand::thread_rng();
        self.all_species.keys().choose(&mut rng).unwrap().clone()
    }
    pub fn get_image_by_tile_type(&self, tile: &str) -> &Image {
        self.all_images
            .get(&self.all_tiles.get(tile).unwrap().image)
            .unwrap()
    }
}
