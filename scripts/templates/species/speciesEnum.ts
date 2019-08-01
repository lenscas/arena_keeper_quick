import { firstToUpper } from "../../imps/str";

export const createSpeciesEnum = (list : string[]) => (
`
pub enum Species {
    ${list.map(firstToUpper).join(",\n\t")}
}
`
)