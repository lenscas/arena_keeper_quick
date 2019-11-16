use crate::modules::structs::TileFeatures;
use crate::structs::point::Point;

/// This structure holds all the information a cell needs
#[derive(Clone)]
pub struct Cell {
    pub cell_type: String,
    pub feature: Option<TileFeatures>,
    pub loc: Point,
}
