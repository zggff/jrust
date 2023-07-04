pub struct ByteStream{
    pub v: Vec<u8>,
    pub i: usize
}

impl From<Vec<u8>> for ByteStream {
    fn from(value: Vec<u8>) -> Self {
        ByteStream { v: value, i: 0 }
        // ByteStream(Box::new(value.into_iter()))
    }
}

impl ByteStream {
    pub fn next(&mut self) -> Option<u8> {
        if self.i < self.v.len() {
            let i = self.i;
            self.i += 1;
            Some(self.v[i])
        } else {
            None
        }
    }
    pub fn advance_by(&mut self, offset: isize) -> Option<()> {
        if offset < 0 && (self.i as isize) < offset {
            return None
        }
        self.i = (self.i as isize + offset) as usize;
        if self.i < self.v.len() {
            Some(())
        } else {
            None
        }
    }
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
