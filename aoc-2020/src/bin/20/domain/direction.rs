#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum CardinalDir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum OrdinalDir {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}
