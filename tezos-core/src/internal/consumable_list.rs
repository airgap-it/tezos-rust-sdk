use core::ops::Range;

use crate::{Error, Result};

pub trait ConsumableList<T> {
    fn inner_value(&self) -> &[T];
    fn consume_first(&mut self) -> Result<T>;
    fn consume_range(&mut self, range: Range<usize>) -> Result<&[T]>
    where
        Self: Sized;
    fn consume_until(&mut self, index: usize) -> Result<&[T]>
    where
        Self: Sized,
    {
        self.consume_range(0..index)
    }
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub struct ConsumableBytes<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> ConsumableBytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes: bytes.as_ref(),
            position: 0,
        }
    }
}

impl<'a> ConsumableList<u8> for ConsumableBytes<'a> {
    fn inner_value(&self) -> &[u8] {
        &self.bytes[self.position..]
    }

    fn consume_first(&mut self) -> Result<u8> {
        if !self.is_empty() {
            let result = self.bytes[self.position];
            self.position += 1;
            return Ok(result);
        }
        Err(Error::InvalidBytes)
    }

    fn consume_range(&mut self, range: Range<usize>) -> Result<&[u8]>
    where
        Self: Sized,
    {
        let start = range.start + self.position;
        let end = range.end + self.position;
        if end > self.bytes.len() {
            return Err(Error::InvalidBytes);
        }
        let result = &self.bytes[start..end];
        self.position = end;
        Ok(result)
    }

    fn len(&self) -> usize {
        self.bytes.len() - self.position
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
