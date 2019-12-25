use crate::modules::structs::TileFeatures;
use crate::structs::point::Point;
use serde::{Deserialize, Serialize};
/// This structure holds all the information a cell needs
#[derive(Clone, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type: String,
    pub feature: Option<TileFeatures>,
    pub loc: Point,
}
