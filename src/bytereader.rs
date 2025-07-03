use std::io::{self, Read, Seek, SeekFrom};

pub struct ByteReader<R: Read + Seek> {
    reader: R,
}


impl<R: Read+Seek> ByteReader<R> {
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

    pub fn fseeko(&mut self, off: u64) -> io::Result<u64> {
        self.reader.seek(SeekFrom::Start(off))
    }
}
