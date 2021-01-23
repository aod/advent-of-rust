pub trait Orientable: Sized {
    fn flip(&mut self);

    fn rotate(&mut self);

    fn orientations(self) -> Orientations<Self> {
        Orientations::from(self)
    }
}

pub struct Orientations<T: Orientable> {
    subject: T,
    orient_count: usize,
}

impl<T: Orientable + Clone> Iterator for Orientations<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.orient_count >= 8 {
            return None;
        }

        if self.orient_count % 2 == 0 {
            self.subject.flip();
        } else {
            self.subject.flip();
            self.subject.rotate();
        }

        self.orient_count += 1;
        Some(self.subject.clone())
    }
}

impl<T: Orientable> From<T> for Orientations<T> {
    fn from(subject: T) -> Self {
        Self {
            subject,
            orient_count: 0,
        }
    }
}
