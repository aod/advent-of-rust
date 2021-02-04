use std::ops::Add;

use super::pocket::SomeCube;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point4(isize, isize, isize, isize);

impl Add for &Point4 {
    type Output = Point4;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Point4 {
    pub fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Self(x, y, z, w)
    }

    pub fn diffs() -> impl Iterator<Item = Self> {
        (-1..=1)
            .flat_map(|x| {
                (-1..=1).flat_map(move |y| {
                    (-1..=1).flat_map(move |z| (-1..=1).map(move |w| (x, y, z, w)))
                })
            })
            .map(|(x, y, z, w)| Self::new(x, y, z, w))
            .filter(|point| point != &Self::new(0, 0, 0, 0))
    }
}

impl SomeCube for Point4 {
    fn nbors(&self) -> Vec<Self> {
        Self::diffs().map(move |diff| self + &diff).collect()
    }
}
