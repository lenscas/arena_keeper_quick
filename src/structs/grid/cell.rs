use super::{
    CellFeature,
    CellType
};
use crate::structs::point::Point;

/// This structure holds all the information a cell needs
#[derive(Clone)]
pub struct Cell {
    pub cell_type : CellType,
    pub feature : Option<CellFeature>,
    pub loc : Point
}