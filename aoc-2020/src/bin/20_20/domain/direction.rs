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

impl PartialEq<&[CardinalDir]> for OrdinalDir {
    fn eq(&self, other: &&[CardinalDir]) -> bool {
        use CardinalDir::*;
        *self
            == match other {
                [East, South] | [South, East] => Self::NorthEast,
                [West, South] | [South, West] => Self::NorthWest,
                [West, North] | [North, West] => Self::SouthEast,
                [East, North] | [North, East] => Self::SouthWest,
                _ => return false,
            }
    }
}
