/*
use crate::generated::tiles::CellType;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Species {
    Human,
    Merfolk,
}
impl Distribution<Species> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Species {
        match rng.gen_range(0, 2) {
            0 => Species::Human,
            1 => Species::Merfolk,
            _ => unreachable!(),
        }
    }
}

impl Species {
    pub fn calc_speed(self, tile: CellType) -> usize {
        match self {
            Species::Human => match tile {
                CellType::Sand => 5,
                CellType::Water => 15,
                CellType::Grass => 1,
                CellType::Stone => 15,
            },
            Species::Merfolk => match tile {
                CellType::Sand => 4,
                CellType::Water => 1,
                CellType::Grass => 4,
                CellType::Stone => 15,
            },
        }
    }
}
impl From<Species> for String {
    fn from(species: Species) -> Self {
        match species {
            Species::Human => "Human",
            Species::Merfolk => "Merfolk",
        }
        .into()
    }
}
*/
