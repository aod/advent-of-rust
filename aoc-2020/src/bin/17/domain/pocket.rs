use std::collections::HashSet;

pub type Cube = super::point::Point3;

/// Pocket consists of **active** cubes.
#[derive(Debug, Default)]
pub struct Pocket(HashSet<Cube>);

impl From<&str> for Pocket {
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

impl Pocket {
    pub fn size(&self) -> usize {
        self.0.len()
    }

    fn active_nbors(&self, cube: Cube) -> usize {
        cube.nbors().filter(|nbor| self.0.contains(&nbor)).count()
    }

    pub fn next(&mut self) {
        let mut next: Self = Default::default();
        let candidates = self
            .0
            .iter()
            .flat_map(Cube::nbors)
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

            // active rule:
            if is_active_cube && (nbors == 3 || nbors == 2) {
                next.0.insert(candidate);
            }
            // inactive rule:
            if !is_active_cube && nbors == 3 {
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
        let pocket = Pocket::from(input);

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
