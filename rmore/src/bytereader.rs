use std::fs::File;
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

    pub fn next_byte(&mut self) -> io::Result<Option<u8>> {
        let mut tmp = [0u8; 1];
        match self.reader.read(&mut tmp)? {
            0 => Ok(None),
            1 => {
                Ok(Some(tmp[0]))
            }
            _ => unreachable!(),
        }
    }
}