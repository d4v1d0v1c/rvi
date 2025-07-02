use std::io::{self, Read};

pub struct ByteReader<R: Read> {
    reader: R,
}


impl<R: Read> ByteReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
        }
    }

    pub fn nextchar(&mut self, byte : &mut u8) -> io::Result<usize> {
        let mut buf = [0u8; 1];
        let n = self.reader.read(&mut buf)?;
        if n == 1 {
            *byte = buf[0];
        }
        Ok(n) // 0 eof
    }
}