use crate::generated::assets::loaded::Images;
/*
/// The type of the cell
#[derive(Clone, Copy)]
pub enum CellType {
    Grass,
    Sand,
    Stone,
    Water,
}
/// used to turn the noise map into the grid. Probably not usefull for anything else
impl From<f64> for CellType {
    fn from(num: f64) -> Self {
        let num = num + 1.0;
        if num >= -2f64 && num <= 0.8f64 {
            CellType::Water
        } else if num >= 0.8f64 && num <= 1f64 {
            CellType::Sand
        } else if num >= 1f64 && num <= 1.3f64 {
            CellType::Grass
        } else if num >= 1.3f64 && num <= 3f64 {
            CellType::Stone
        } else {
            println!("shit?");
            unreachable!()
        }
    }
}
impl CellType {
    pub fn get_image(self) -> Images {
        match self {
            CellType::Grass => "base/tiles/images/grass".into(),
            CellType::Sand => "base/tiles/images/sand".into(),
            CellType::Stone => "base/tiles/images/stone".into(),
            CellType::Water => "base/tiles/images/water".into(),
        }
    }
}
*/