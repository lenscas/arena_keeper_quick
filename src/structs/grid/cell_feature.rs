/*
use crate::structs::character::CharId;

/// The feature this cell has, for example if a wall is placed on top of it.
#[derive(Clone, PartialEq)]
pub enum CellFeature {
    None,
    Wall,
    Bed(Option<CharId>),
}

impl CellFeature {
    pub fn can_walk(&self) -> bool {
        match self {
            CellFeature::None => true,
            CellFeature::Wall => false,
            CellFeature::Bed(_) => true,
        }
    }
    pub fn can_sleep(&self, id: CharId) -> bool {
        match self {
            CellFeature::None => false,
            CellFeature::Wall => false,
            CellFeature::Bed(Some(x)) => *x == id,
            CellFeature::Bed(None) => true,
        }
    }
    pub fn get_speed_penalty(&self) -> Option<usize> {
        match self {
            CellFeature::Bed(_) => Some(5),
            _ => None,
        }
    }
}
*/
