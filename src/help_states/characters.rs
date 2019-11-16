use crate::{
    modules::structs::ModulesContainer,
    structs::{grid::Field, BuyableCharacter, Character, FullContext},
};

#[derive(Default)]
pub struct Characters {
    characters: Vec<Character>,
}
impl Characters {
    pub fn new() -> Self {
        Self {
            characters: Vec::new(),
        }
    }
    pub fn add_character(&mut self, new_char: BuyableCharacter) {
        self.characters
            .push(Character::from_bought_char(self.characters.len(), new_char));
    }
    #[cfg(target_arch = "wasm32")]
    fn update_paralel(&mut self, grid: &Field, mods: &ModulesContainer) {
        self.characters
            .iter_mut()
            .for_each(|v| v.update_par(grid, &mods.all_species, &mods.all_tiles));
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn update_paralel(&mut self, grid: &Field, mods: &ModulesContainer) {
        use rayon::prelude::*;
        self.characters
            .par_iter_mut()
            .for_each(|v| v.update_par(grid, mods));
    }
    pub fn update(&mut self, grid: &mut Field, mods: &ModulesContainer) {
        self.update_paralel(grid, mods);
        self.characters
            .iter_mut()
            .for_each(|v| v.update(grid, mods));
    }
    pub fn render(&mut self, context: &mut FullContext) {
        self.characters.iter_mut().for_each(|v| v.render(context));
    }
}
