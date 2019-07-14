use crate::structs::character::CharId;

/// The feature this cell has, for example if a wall is placed on top of it.
#[derive(Clone, PartialEq)]
pub enum CellFeature {
    None,
    Wall,
    Bed(Option<CharId>),
}
