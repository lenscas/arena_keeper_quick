use crate::structs::CharId;
use rand::seq::{IteratorRandom, SliceRandom};
use std::{collections::HashMap, path::Path};

use quicksilver::graphics::Image;

#[derive(Default)]
pub struct Module {
    features: HashMap<String, TileFeatureRaw>,
    species: HashMap<SpeciesType, SpeciesConf>,
    pub images: HashMap<String, Image>,
    tiles: Option<TilesConf>,
}
impl Module {
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
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
    pub fn set_features(&mut self, name: String, features: TileFeatureRaw) {
        self.features.insert(name, features);
    }
    pub fn add_to_all_mods(mut self, all_mods: &mut ModulesContainer) -> HashMap<String, Image> {
        let all_images = self.images.drain().collect();
        all_mods.add_module(self);
        all_images
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeciesKinds {
    Land,
    Water,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeciesConf {
    kind: SpeciesKinds,
    speeds: HashMap<String, usize>,
    base_speeds: Vec<usize>,
    pub name: SpeciesType,
    possible_names: Vec<String>,
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
    pub fn get_image(&self) -> &str {
        match &self {
            Tile::BaseTile { image, .. } | Tile::ExtendingTile { image, .. } => image,
        }
    }
    pub fn get_speed(
        &self,
        tile: &str,
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
fn def_false() -> bool {
    false
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileFeatureRaw {
    pub image: String,
    pub name: String,
    pub speed_penalty: Option<usize>,
    #[serde(default = "def_false")]
    pub is_transparent: bool,
    pub is_ownable: bool,
    pub is_bed: bool,
    pub can_walk_on: bool,
}

#[derive(Clone)]
pub enum TileFeatures {
    Ownable { owner: Option<CharId>, tile: String },
    NotOwnable(String),
}

impl TileFeatures {
    pub fn get_feature_name(&self) -> &str {
        match self {
            TileFeatures::Ownable { tile, .. } | TileFeatures::NotOwnable(tile) => &tile,
        }
    }
    pub fn set_owned(&mut self, id: Option<CharId>) {
        if let TileFeatures::Ownable { owner, .. } = self {
            *owner = id
        }
    }
    pub fn can_walk(&self, mods: &ModulesContainer) -> bool {
        match self {
            TileFeatures::NotOwnable(tile) | TileFeatures::Ownable { tile, .. } => {
                mods.get_feature(tile).can_walk_on
            }
        }
    }
    pub fn is_owned_by(&self, id: CharId) -> bool {
        match self {
            TileFeatures::NotOwnable(_) => false,
            TileFeatures::Ownable {
                owner: Some(owner), ..
            } => *owner == id,
            TileFeatures::Ownable { owner: None, .. } => false,
        }
    }
    pub fn can_sleep(&self, id: CharId, mods: &ModulesContainer) -> bool {
        match self {
            TileFeatures::NotOwnable(tile) => mods.get_feature(tile).is_bed,
            TileFeatures::Ownable { tile, owner } => {
                let tile = mods.get_feature(tile);
                match (tile, owner) {
                    (TileFeatureRaw { is_bed: true, .. }, Some(owner)) => *owner == id,
                    (TileFeatureRaw { is_bed: false, .. }, _) => false,
                    (TileFeatureRaw { is_bed: true, .. }, None) => true,
                }
            }
        }
    }
    pub fn get_speed_penalty(&self, mods: &ModulesContainer) -> Option<usize> {
        match self {
            TileFeatures::NotOwnable(tile) | TileFeatures::Ownable { tile, .. } => {
                mods.get_feature(tile).speed_penalty
            }
        }
    }
    pub fn get_image<'a>(&self, mods: &'a ModulesContainer) -> &'a str {
        match self {
            TileFeatures::NotOwnable(tile) | TileFeatures::Ownable { tile, .. } => {
                &mods.get_feature(tile).image
            }
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TilesConf {
    generate_chances: HashMap<String, Tile>,
}

#[derive(PartialEq, Eq, Hash, serde::Deserialize, Clone)]
pub struct SpeciesType(String);
impl SpeciesType {
    pub fn get_speed_on_tile(
        &self,
        species: &HashMap<SpeciesType, SpeciesConf>,
        tiles: &HashMap<String, Tile>,
        tile: &str,
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
    pub all_features: HashMap<String, TileFeatureRaw>,
}
impl ModulesContainer {
    pub fn get_species(&self, species: &SpeciesType) -> &SpeciesConf {
        self.all_species.get(species).unwrap()
    }
    pub fn get_tile(&self, tile: &str) -> &Tile {
        self.all_tiles.get(tile).unwrap()
    }
    pub fn get_feature(&self, feature: &str) -> &TileFeatureRaw {
        self.all_features
            .get(feature)
            .unwrap_or_else(|| panic!("{:?} is not a loaded feature", feature))
    }
    pub fn add_module(&mut self, module: Module) {
        self.all_species.extend(module.species);
        self.all_features.extend(module.features);
        if let Some(tiles) = module.tiles {
            self.all_tiles.extend(tiles.generate_chances)
        }
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
    pub fn get_random_base_speed(&self, species: &SpeciesType) -> usize {
        let mut rng = rand::thread_rng();
        *self
            .all_species
            .get(species)
            .unwrap()
            .base_speeds
            .choose(&mut rng)
            .unwrap()
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
            .possible_names
            .choose(&mut rng)
            .unwrap()
            .clone()
    }
    pub fn get_random_species(&self) -> SpeciesType {
        let mut rng = rand::thread_rng();
        self.all_species.keys().choose(&mut rng).unwrap().clone()
    }
}
