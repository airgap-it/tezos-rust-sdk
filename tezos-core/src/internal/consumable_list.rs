use std::ops::Range;

use crate::{Error, Result};

pub trait ConsumableList<T> {
    fn consume_at(&mut self, index: usize) -> Result<T>;
    fn consume_range(&mut self, range: Range<usize>) -> Result<Self>
    where
        Self: Sized;
    fn consume_until(&mut self, index: usize) -> Result<Self>
    where
        Self: Sized,
    {
        self.consume_range(0..index)
    }
    fn len(&self) -> usize;
}

impl<T> ConsumableList<T> for Vec<T> {
    fn consume_at(&mut self, index: usize) -> Result<T> {
        if index < self.len() {
            return Ok(self.remove(index));
        }
        Err(Error::InvalidBytes)
    }

    fn consume_range(&mut self, range: Range<usize>) -> Result<Self> {
        if range.end <= self.len() {
            return Ok(self.splice(range, vec![]).collect());
        }
        Err(Error::InvalidBytes)
    }

    fn len(&self) -> usize {
        self.len()
    }
}
