use std::ops::Add;

use super::pocket::SomeCube;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3(isize, isize, isize);

impl Add for &Point3 {
    type Output = Point3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Point3 {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self(x, y, z)
    }

    pub fn diffs() -> impl Iterator<Item = Self> {
        (-1..=1)
            .flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (x, y, z))))
            .map(|(x, y, z)| Self::new(x, y, z))
            .filter(|point| point != &Point3::new(0, 0, 0))
    }
}

impl SomeCube for Point3 {
    fn nbors(&self) -> Vec<Self> {
        Self::diffs().map(move |diff| self + &diff).collect()
    }
}
