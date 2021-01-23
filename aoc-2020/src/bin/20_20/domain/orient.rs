pub(crate) trait Orientable {
    fn flip(&mut self);

    fn rotate(&mut self);
}

pub(crate) struct Orientations<T>
where
    T: Orientable,
{
    subject: T,
    orient_count: usize,
}

impl<T> Iterator for Orientations<T>
where
    T: Orientable + Clone,
{
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

impl<T> From<T> for Orientations<T>
where
    T: Orientable,
{
    fn from(subject: T) -> Self {
        Self {
            subject,
            orient_count: 0,
        }
    }
}
