import { firstToUpper } from "../../imps/str";

export const createSpeciesEnum = (list : string[]) => (
`
use rand::Rng;
use rand::distributions::Standard;
use rand::distributions::Distribution;

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
pub enum Species {
    ${list.map(firstToUpper).join(",\n\t")}
}
impl Distribution<Species> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Species {
        match rng.gen_range(0, ${list.length}) {
            ${list.map(
                (v,k)=>
`                   ${k} => Species::${firstToUpper(v)},`
            ).join("\n")}
            _ => unreachable!()
        }
    }
}
`
)