import { firstToUpper } from "../imps/str";

export const tileEnum = (tiles: {images:string[],conf :any}) => (
`
use crate::generated::assets::loaded::Images;

/// The type of the cell
#[derive(Clone,Copy)]
pub enum CellType {
    ${tiles.images.map(firstToUpper).join(",\n\t")}
}
/// used to turn the noise map into the grid. Probably not usefull for anything else
impl From<f64> for CellType {
    fn from(num: f64) -> Self {
        let num = num + 1.0;
        ${Object.keys(tiles.conf.generateChances).map(
            v=>{
                const weights = tiles.conf.generateChances[v]
                return `if num >= ${weights.start}f64 && num <= ${weights.end}f64 {
                    CellType::${v}
                }`
            }
        ).join(" else ")}
        else {
            println!("shit?");
            unreachable!()
        }
    }
}
impl CellType {
    pub fn get_image(self) -> Images {
        match self {
            ${tiles.images.map(firstToUpper).map(v=>`CellType::${v} => Images::GeneratedTiles${v}`).join(",\n\t\t\t")}
        }
    }
}
`
)