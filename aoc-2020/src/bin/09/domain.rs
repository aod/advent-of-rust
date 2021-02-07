use std::collections::VecDeque;

#[derive(Debug)]
pub struct XMAS {
    xs: VecDeque<usize>,
}

impl XMAS {
    pub fn new(preamble: &[usize]) -> Self {
        Self {
            xs: preamble.to_owned().into(),
        }
    }

    pub fn next(&mut self, val: usize) -> bool {
        if !self.is_valid(val) {
            return false;
        }

        self.xs.pop_front();
        self.xs.push_back(val);

        true
    }

    pub fn is_valid(&self, val: usize) -> bool {
        self.xs.iter().enumerate().any(|(i, x)| {
            self.xs
                .iter()
                .enumerate()
                .any(|(j, y)| i != j && x + y == val)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let cypher = XMAS::new((1..=25).collect::<Vec<usize>>().as_slice());

        assert!(cypher.is_valid(25));
        assert!(cypher.is_valid(49));
        assert!(!cypher.is_valid(100));
        assert!(!cypher.is_valid(50));
    }

    #[test]
    fn test_example_2() {
        let mut preamble = (1..=25).collect::<Vec<usize>>();
        preamble[0] = 20;
        preamble[19] = 1;
        let mut cypher = XMAS::new(preamble.as_slice());

        assert!(cypher.next(45));

        assert!(cypher.is_valid(26));
        assert!(!cypher.is_valid(65));
        assert!(cypher.is_valid(64));
        assert!(cypher.is_valid(66));
    }
}
