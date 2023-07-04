use std::ops::{Deref, DerefMut};

pub struct ByteStream(Box<dyn Iterator<Item = u8>>);

impl Deref for ByteStream {
    type Target = Box<dyn Iterator<Item = u8>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<u8>> for ByteStream {
    fn from(value: Vec<u8>) -> Self {
        ByteStream(Box::new(value.into_iter()))
    }
}

impl ByteStream {
    pub fn next_u1(&mut self) -> Option<u8> {
        self.next()
    }

    // INFO: here we return usize because 2 byte variables are used for indexing. In rust indexes are always usize
    pub fn next_u2(&mut self) -> Option<usize> {
        let a = (self.next()? as usize) << 8;
        let b = self.next()? as usize;
        Some(a | b)
    }

    // INFO: here we return usize because 4 byte variables are used for indexing. In rust indexes are always usize
    pub fn next_u4(&mut self) -> Option<usize> {
        let a = (self.next()? as usize) << (8 * 3);
        let b = (self.next()? as usize) << (8 * 2);
        let c = (self.next()? as usize) << 8;
        let d = self.next()? as usize;
        Some(a | b | c | d)
    }
}
