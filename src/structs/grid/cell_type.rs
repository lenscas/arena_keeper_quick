/// The type of the cell
#[derive(Clone)]
pub enum CellType {
    Ground,
    Water,
    Grass,
    Stone,
}
/// used to turn the noise map into the grid. Probably not usefull for anything else
impl From<f64> for CellType {
    fn from(num: f64) -> Self {
        let num = num + 1.0;
        if num <= 0.6 {
            CellType::Water
        } else if num > 0.6 && num <= 1.0 {
            CellType::Grass
        } else if num > 1.0 && num <= 1.3 {
            CellType::Ground
        } else {
            CellType::Stone
        }
    }
}
