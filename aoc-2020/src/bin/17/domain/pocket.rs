use std::{collections::HashSet, hash::Hash};

use super::{point3::Point3, point4::Point4};

pub type Cube = Point3;
pub type HyperCube = Point4;

pub trait SomeCube: Sized + PartialEq + Eq + Hash + Copy {
    fn nbors(&self) -> Vec<Self>;
}

/// Pocket consists of **active** cubes.
#[derive(Debug, Default)]
pub struct Pocket<T: SomeCube>(HashSet<T>);

impl From<&str> for Pocket<Cube> {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .enumerate()
                .flat_map(|(x, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, state)| state == &'#')
                        .map(move |(y, _)| Cube::new(x as isize, y as isize, 0))
                })
                .collect(),
        )
    }
}

impl From<&str> for Pocket<HyperCube> {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .enumerate()
                .flat_map(|(x, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, state)| state == &'#')
                        .map(move |(y, _)| HyperCube::new(x as isize, y as isize, 0, 0))
                })
                .collect(),
        )
    }
}

impl<T: SomeCube> Pocket<T> {
    fn new() -> Self {
        Self(Default::default())
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    fn active_nbors(&self, cube: T) -> usize {
        cube.nbors()
            .iter()
            .filter(|nbor| self.0.contains(&nbor))
            .count()
    }

    pub fn next(&mut self) {
        let mut next: Self = Self::new();
        let candidates = self
            .0
            .iter()
            .flat_map(T::nbors)
            .chain(self.0.iter().copied());

        for candidate in candidates {
            let nbors = self.active_nbors(candidate);
            let is_active_cube = self.0.contains(&candidate);

            /*
             * During a cycle, all cubes simultaneously change their state
             * according to the following rules:

             * - If a cube is active and exactly 2 or 3 of its neighbors are also
             *   active, the cube remains active. Otherwise, the cube becomes
             *   inactive.
             * - If a cube is inactive but exactly 3 of its neighbors are
             *   active, the cube becomes active. Otherwise, the cube remains
             *   inactive.
            */

            if nbors == 3 || is_active_cube && nbors == 2 {
                next.0.insert(candidate);
            }
        }

        *self = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_works() {
        let input = "\
            .#.\n\
            ..#\n\
            ###";
        let pocket = Pocket::<Cube>::from(input);

        assert_eq!(
            pocket.0,
            vec![
                Cube::new(0, 1, 0),
                Cube::new(1, 2, 0),
                Cube::new(2, 0, 0),
                Cube::new(2, 1, 0),
                Cube::new(2, 2, 0),
            ]
            .into_iter()
            .collect()
        );
    }
}
