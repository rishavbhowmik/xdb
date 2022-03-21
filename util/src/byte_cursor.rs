use std::io::Error;
use std::io::ErrorKind;

pub struct Cursor {
    bytes: Vec<u8>,
    position: usize,
}

impl Cursor {
    pub fn new(bytes: &[u8]) -> Cursor {
        Cursor {
            bytes: bytes.to_vec(),
            position: 0,
        }
    }

    pub fn remaining_bytes(&self) -> usize {
        self.bytes.len() - self.position
    }

    // pub fn remaining_slice(&self) -> &[u8] {
    //     &self.bytes[self.position..]
    // }

    // pub fn get_ref(&self) -> &[u8] {
    //     &self.bytes
    // }

    // pub fn get_mut(&mut self) -> &mut Vec<u8> {
    //     &mut self.bytes
    // }

    // pub fn position(&self) -> usize {
    //     self.position
    // }

    // pub fn set_position(&mut self, position: usize) {
    //     self.position = position;
    // }

    // pub fn advance(&mut self, len: usize) {
    //     self.position += len;
    // }

    pub fn consume(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        if self.remaining_bytes() < len {
            return Err(Error::new(ErrorKind::UnexpectedEof, "No bytes remaining"));
        }
        self.position += len;
        Ok(self.bytes[(self.position - len)..(self.position)].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cusor_consume() {

    }
}