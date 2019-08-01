import { firstToUpper } from "../../imps/str";

export const createCharacterNames = (list : Array<{names:string[],specie:string}>) => (
`
use super::species::Species;
use rand::seq::SliceRandom;

pub fn get_random_name(specie : Species) -> String {
    let mut rng = rand::thread_rng();
    String::from(*match specie {
        ${list
            .map(
                v=>`Species::${
                    firstToUpper(v.specie)
                } => vec![${
                    v.names.map(
                        v=>`"${v}"`).join(",")
                }]`
            ).join(",\n\t\t")
        }
    }.choose(&mut rng).unwrap())
}
`
)
export const createSpecieImages = (list : Array<{specie:string,images:string[]}>) => (
`
use super::species::Species;
use crate::generated::assets::loaded::Images;
use rand::seq::SliceRandom;

pub fn get_random_image(specie : Species) -> Images {
    let mut rng = rand::thread_rng();
    *match specie {
        ${list
            .map(
                v=>`Species::${
                    firstToUpper(v.specie)
                } => vec![${
                    v.images.map(
                        v=>`Images::${v}`).join(",")
                }]`
            ).join(",\n\t\t")
        }
    }.choose(&mut rng).unwrap()
}
`
)