use rand::seq::{IteratorRandom, SliceRandom};
use std::{collections::HashMap, path::Path};

use quicksilver::graphics::Image;

#[derive(Default)]
pub struct Module {
    species: HashMap<SpeciesType, SpeciesConf>,
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
    pub fn set_species(&mut self, name: SpeciesType, species: SpeciesConf) {
        self.species.insert(name, species);
    }
    pub fn set_tiles(&mut self, tiles: TilesConf) {
        self.tiles = Some(tiles);
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeciesKinds {
    Land,
    Water,
}
#[derive(serde::Deserialize)]
pub struct SpeciesConf {
    kind: SpeciesKinds,
    speeds: HashMap<String, usize>,
    pub name: SpeciesType,
    possibleNames: Vec<String>,
    images: Vec<ImageName>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Tile {
    BaseTile {
        image: String,
        end: f64,
        speeds: HashMap<SpeciesKinds, usize>,
    },
    ExtendingTile {
        end: f64,
        image: String,
        extend: TileType,
        speeds: Option<HashMap<SpeciesKinds, usize>>,
    },
}
impl Tile {
    pub fn get_end(&self) -> f64 {
        match &self {
            Tile::BaseTile { end, .. } | Tile::ExtendingTile { end, .. } => *end,
        }
    }
    pub fn get_image(&self) -> &String {
        match &self {
            Tile::BaseTile { image, .. } | Tile::ExtendingTile { image, .. } => image,
        }
    }
    pub fn get_speed(
        &self,
        tile: &TileType,
        kind: SpeciesKinds,
        overwrites: &HashMap<String, usize>,
        tiles: &HashMap<String, Tile>,
    ) -> usize {
        overwrites.get(tile).copied().unwrap_or_else(|| match self {
            Tile::BaseTile { speeds, .. } => *speeds.get(&kind).unwrap(),
            Tile::ExtendingTile { extend, speeds, .. } => speeds
                .as_ref()
                .and_then(|v| v.get(&kind).copied())
                .unwrap_or_else(|| {
                    tiles
                        .get(extend)
                        .unwrap()
                        .get_speed(extend, kind, overwrites, tiles)
                }),
        })
    }
}

#[derive(serde::Deserialize)]
pub struct TilesConf {
    generateChances: HashMap<String, Tile>,
}

#[derive(PartialEq, Eq, Hash, serde::Deserialize, Clone)]
pub struct SpeciesType(String);
impl SpeciesType {
    pub fn get_speed_on_tile(
        &self,
        species: &HashMap<SpeciesType, SpeciesConf>,
        tiles: &HashMap<String, Tile>,
        tile: &TileType,
    ) -> usize {
        let species = species.get(self).unwrap();
        tiles
            .get(tile)
            .unwrap()
            .get_speed(tile, species.kind, &species.speeds, &tiles)
    }
}

impl<'a> From<&'a SpeciesType> for &'a str {
    fn from(from: &'a SpeciesType) -> &'a str {
        &from.0
    }
}

impl From<SpeciesType> for String {
    fn from(from: SpeciesType) -> String {
        from.0
    }
}

pub type ImageName = String;
pub type TileType = String;

#[derive(Default)]
pub struct ModulesContainer {
    //modules : Vec<Module>,
    pub all_species: HashMap<SpeciesType, SpeciesConf>,
    pub all_tiles: HashMap<String, Tile>,
    all_images: HashMap<ImageName, Image>,
}
impl ModulesContainer {
    pub fn get_species(&self, species: &SpeciesType) -> &SpeciesConf {
        self.all_species.get(species).unwrap()
    }
    pub fn get_tile(&self, tile: &TileType) -> &Tile {
        self.all_tiles.get(tile).unwrap()
    }
    pub fn add_module(&mut self, module: Module) {
        self.all_species.extend(module.species);
        if let Some(tiles) = module.tiles {
            self.all_tiles.extend(tiles.generateChances)
        }
        self.all_images.extend(module.images);
    }
    pub fn get_random_image_for_species(&self, species: &SpeciesType) -> ImageName {
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
                if tile.get_end() > num {
                    if found.is_none() || found.unwrap().1.get_end() - num > tile.get_end() - num {
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
    pub fn get_random_name_for_species(&self, species: &SpeciesType) -> String {
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
            .get(self.all_tiles.get(tile).unwrap().get_image())
            .unwrap()
    }
}
