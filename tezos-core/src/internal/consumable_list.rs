use std::ops::Range;

pub trait ConsumableList<T> {
    fn consume_at(&mut self, index: usize) -> Option<T>;
    fn consume_range(&mut self, range: Range<usize>) -> Self;
    fn consume_until(&mut self, index: usize) -> Self
    where
        Self: Sized,
    {
        self.consume_range(0..index)
    }
    fn len(&self) -> usize;
}

impl<T> ConsumableList<T> for Vec<T> {
    fn consume_at(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            return Some(self.remove(index));
        }
        None
    }

    fn consume_range(&mut self, range: Range<usize>) -> Self {
        self.splice(range, vec![]).collect()
    }

    fn len(&self) -> usize {
        self.len()
    }
}
