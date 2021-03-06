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
        let mut cursor = Cursor::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(cursor.remaining_bytes(), 10);
        assert_eq!(cursor.consume(5).unwrap(), [1, 2, 3, 4, 5]);
        assert_eq!(cursor.remaining_bytes(), 5);
        assert_eq!(cursor.consume(5).unwrap(), [6, 7, 8, 9, 10]);
        assert_eq!(cursor.remaining_bytes(), 0);
        assert!(cursor.consume(1).is_err());
        let mut cursor = Cursor::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        assert_eq!(cursor.remaining_bytes(), 12);
        assert_eq!(cursor.consume(5).unwrap(), [1, 2, 3, 4, 5]);
        assert_eq!(cursor.remaining_bytes(), 7);
        assert_eq!(cursor.consume(5).unwrap(), [6, 7, 8, 9, 10]);
        assert_eq!(cursor.remaining_bytes(), 2);
        assert_eq!(cursor.consume(2).unwrap(), [11, 12]);
        assert_eq!(cursor.remaining_bytes(), 0);
        assert!(cursor.consume(1).is_err());
        assert!(cursor.consume(0).is_ok());
        assert!(cursor.consume(5).is_err());
    }
}
