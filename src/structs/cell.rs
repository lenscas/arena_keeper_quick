/// The type of the cell
#[derive(Clone)]
pub enum CellType {
    Ground,
    Water,
    Grass,
    Stone
}
impl From<f64> for CellType {
    fn from(num : f64) -> Self {
        let num = num +1.0;
        if num <=0.6 {
            CellType::Water
        } else if num > 0.6 && num <=1.0 {
            CellType::Grass
        } else if num > 1.0 && num <=1.3 {
            CellType::Ground
        } else {
            CellType::Stone
        }
    }
}
/// The feature this cell has, for example if a wall is placed on top of it.
#[derive(Clone)]
pub enum CellFeature {
    Wall
}

/// This structure holds all the information a cell needs
#[derive(Clone)]
pub struct Cell {
    pub cell_type : CellType,
    pub feature : Option<CellFeature>,
    pub x : isize,
    pub y : isize
}