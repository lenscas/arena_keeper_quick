use crate::{
    help_states::Characters,
    modules::structs::ModulesContainer,
    structs::{grid::Field, BuyableCharacter},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveableState {
    pub grid: Field,
    pub bought_characters: Characters,
    pub buyable_charachters: Vec<BuyableCharacter>,
    pub money: u32,
}
impl SaveableState {
    pub fn new(grid_size: (usize, usize), seed: u32, mods: &ModulesContainer) -> Self {
        Self {
            grid: Field::new(grid_size.0, grid_size.1, seed, mods),
            bought_characters: Characters::new(),
            buyable_charachters: (0..3).map(|_| BuyableCharacter::new(mods)).collect(),
            money: 20,
        }
    }
}
