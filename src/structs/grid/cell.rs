use super::{CellFeature};
use crate::{
    generated::tiles::CellType,
    structs::point::Point
};

/// This structure holds all the information a cell needs
#[derive(Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub feature: CellFeature,
    pub loc: Point,
}
