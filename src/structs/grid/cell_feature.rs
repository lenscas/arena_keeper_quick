use crate::structs::character::CharId;

/// The feature this cell has, for example if a wall is placed on top of it.
#[derive(Clone)]
pub enum CellFeature {
    Wall,
    Bed(Option<CharId>),
}
