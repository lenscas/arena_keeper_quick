/// The type of the cell
#[derive(Clone)]
pub enum CellType {
    Ground,
    Water,
    Grass,
    Stone,
    Clicked
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