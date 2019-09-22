use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

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
	_speeds: Value,
	pub name: String,
	possibleNames: Vec<String>,
	images: Vec<ImageName>,
}

#[derive(serde::Deserialize)]
pub struct TilesConf {
	generateChances: Value,
}

pub type SpeciesType = String;
pub type ImageName = String;
pub type TileType = String;

#[derive(Default)]
pub struct ModulesContainer {
	//modules : Vec<Module>,
	all_species: HashMap<SpeciesType, SpeciesConf>,
	all_tiles: Vec<TilesConf>,
	all_images: HashMap<ImageName, Image>,
}
impl ModulesContainer {
	pub fn add_module(&mut self, module: Module) {
		self.all_species.extend(module.species);
		self.all_tiles.extend(module.tiles);
		self.all_images.extend(module.images);
		//self.modules.push(module);
	}
	pub fn get_random_image_for_species(&self, species: &str) -> ImageName {
		let mut rng = rand::thread_rng();
		println!("get image for : {}", species);
		self.all_species
			.get(species)
			.unwrap()
			.images
			.choose(&mut rng)
			.unwrap()
			.clone()
	}
	pub fn get_random_name_for_species(&self, species: &str) -> String {
		let mut rng = rand::thread_rng();
		println!("get name for : {}", species);
		self.all_species
			.get(species)
			.unwrap()
			.possibleNames
			.choose(&mut rng)
			.unwrap()
			.clone()
	}
	pub fn get_image_by_name(&self, name: &str) -> &Image {
		println!("get image by name : {}", name);
		self.all_images.get(name).unwrap()
	}
	pub fn get_random_species(&self) -> SpeciesType {
		let mut rng = rand::thread_rng();
		self.all_species.keys().choose(&mut rng).unwrap().clone()
	}
}
