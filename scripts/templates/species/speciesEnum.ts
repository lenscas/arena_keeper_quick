import { firstToUpper } from "../../imps/str";
import { Conf } from "../../types/conf";

export const createSpeciesEnum = (list : Array<{type: string, config : Conf}>) => (
`
use crate::generated::tiles::CellType;
use rand::Rng;
use rand::distributions::Standard;
use rand::distributions::Distribution;

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
pub enum Species {
    ${list.map(v=>firstToUpper(v.type)).join(",\n\t")}
}
impl Distribution<Species> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Species {
        match rng.gen_range(0, ${list.length}) {
            ${list.map(
                (v,k)=>
`                   ${k} => Species::${firstToUpper(v.type)},`
            ).join("\n")}
            _ => unreachable!()
        }
    }
}

impl Species {
    pub fn calc_speed(self, tile : CellType) -> usize {
        match self {
            ${list
                .map(
                    v=> `Species::${firstToUpper(v.type)} => match tile {
                        ${Object.keys(v.config.speeds)
                            .map(x=> `CellType::${x}=> ${v.config.speeds[x]}`)
                            .join(",\n\t\t\t\t")
                        }
                    }`
                )
            }
        }
    }
}
`
)