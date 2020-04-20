use crate::error::NbsError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io;

pub trait ReadStringExt: ReadBytesExt {
    fn read_string(&mut self) -> Result<String, NbsError> {
        let len = self.read_i32::<LittleEndian>()?;
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
        buffer.resize(len as usize, 0u8);
        self.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

pub trait WriteStringExt: WriteBytesExt {
    fn write_string(&mut self, s: &str) -> io::Result<()> {
        self.write_i32::<LittleEndian>(s.len() as i32)?;
        self.write_all(s.as_bytes())?;
        Ok(())
    }
}

impl<R> ReadStringExt for R where R: ReadBytesExt {}
impl<W> WriteStringExt for W where W: WriteBytesExt {}
