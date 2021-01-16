#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Cup(pub usize);

#[derive(Debug)]
pub(super) struct Cups(pub Vec<Cup>);

impl From<&str> for Cups {
    fn from(input: &str) -> Self {
        Self(
            input
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .map(Cup)
                .collect(),
        )
    }
}

/// Implementation for the Crab Cups game.
///
/// The structure is a sinlge Vec with cup's next values stored next to it as
/// cup's index + 1. The first index always points to the current cup's next cup.
/// So to get the actual current cup you do `cups[cups[0] - 1]`. To retrieve the
/// next cup of a cup the algorithm is: `cups[cups[cup * 2] - 1]`. See below for
/// an attempt at explaining it in some kind of ASCII art.
///
/// ```
///        ┌───────────────── Start: Points to current cup's next cup.
///        │ ┌───┬───┬───┬──── Cups: The cups values
///        │ │ ┌─┼─┬─┼─┬─┼─┬─── Nexts: The next cup's index
///        │ │ │ │ │ │ │ │ │
/// Index: 0 1 2 3 4 5 6 7 8
///   Cup:   1   2   3   4
///  Next: 6   6   2   8   4
///
/// Cups: [3 4 2 1]
/// ```
#[derive(Debug)]
pub(super) struct CrabCups {
    cups: Vec<usize>,
    length: usize,
}

impl CrabCups {
    pub(super) fn new(cups: &Cups) -> Self {
        Self::with_length(cups, 0)
    }

    pub(super) fn with_length(cups: &Cups, length: usize) -> Self {
        let cups_length = cups.0.len();
        let length = cups_length.max(length);
        let mut inner: Vec<usize> = vec![0; length * 2 + 1];
        inner[0] = cups.0[0].0 * 2;

        let cups: Vec<usize> = cups
            .0
            .iter()
            .copied()
            .map(|v| v.0)
            .chain(cups_length + 1..=length)
            .collect();

        for xs in cups[..].windows(2) {
            let idx = xs[0] * 2;
            inner[idx] = xs[1] * 2;
            inner[idx - 1] = xs[0];
        }

        let last = cups.last().unwrap();
        let idx = last * 2;
        inner[idx] = cups[0] * 2;
        inner[idx - 1] = *last;

        Self {
            cups: inner,
            length,
        }
    }

    pub(super) fn next(&self, cup: usize) -> usize {
        self.cups[self.cups[cup * 2] - 1]
    }

    fn curr(&self) -> usize {
        self.cups[self.cups[0] - 1]
    }

    fn dest_cup(&self) -> usize {
        let mut target = self.curr();
        let p1 = self.next(target);
        let p2 = self.next(p1);
        let p3 = self.next(p2);

        loop {
            target -= 1;
            if target <= 0 {
                target = self.length;
            }

            if p1 != target && p2 != target && p3 != target {
                break;
            }
        }

        target
    }

    pub(super) fn do_move(&mut self) {
        let curr_cup = self.curr();
        let dest_cup = self.dest_cup();

        let p1 = self.next(curr_cup);
        let p3 = self.next(self.next(p1));

        // Current cup's next cup is gonna be whatever the next cup is for p3.
        self.cups[curr_cup * 2] = self.cups[p3 * 2];
        // p3's next up will be whatever the next cup for dest cup is.
        self.cups[p3 * 2] = self.cups[dest_cup * 2];
        // Dest cup's next cup is gonna be p1.
        self.cups[dest_cup * 2] = p1 * 2;

        self.cups[0] = self.cups[curr_cup * 2];
    }

    pub(super) fn label(&self) -> String {
        let mut result = String::new();

        let mut cup = self.next(1);
        while cup != 1 {
            result.push_str(&cup.to_string());
            cup = self.next(cup);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crab_cups_new_works() {
        let cups = Cups(vec![Cup(3), Cup(4), Cup(2), Cup(1)]);
        let cc = CrabCups::new(&cups);
        assert_eq!(cc.cups, vec![6, 1, 6, 2, 2, 3, 8, 4, 4]);
    }

    #[test]
    fn crab_cups_next_works() {
        let cups = Cups(
            vec![3, 8, 9, 1, 2, 5, 4, 6, 7]
                .into_iter()
                .map(Cup)
                .collect(),
        );
        let cc = CrabCups::new(&cups);
        assert_eq!(cc.dest_cup(), 2);
    }

    #[test]
    fn crab_cups_do_move_works() {
        let cups = Cups(
            vec![3, 8, 9, 1, 2, 5, 4, 6, 7]
                .into_iter()
                .map(Cup)
                .collect(),
        );
        let mut cc = CrabCups::new(&cups);

        cc.do_move();
        assert_eq!(cc.dest_cup(), 7);
    }
}
