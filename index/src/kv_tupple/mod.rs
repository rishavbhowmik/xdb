use util::byte_cursor::Cursor;
use util::error::Error;

mod kv_tupple_errors;

pub struct KVTupple {
    key: Vec<u8>,
    value: Vec<u8>,
}

impl KVTupple {
    pub fn new(key: &[u8], value: &[u8]) -> Self {
        KVTupple {
            key: key.to_vec(),
            value: value.to_vec(),
        }
    }

    pub fn from_byte_cursor(cursor: &mut Cursor) -> Result<Self, Error> {
        // - parse key length
        let byte_array = cursor.consume(4);
        if byte_array.is_err() {
            if byte_array.err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(kv_tupple_errors::from_bytes_parse_key_length_invalid_eof());
            } else {
                return Err(kv_tupple_errors::from_bytes_parse_key_length_invalid_bytes());
            }
        }
        let byte_array = byte_array.unwrap();
        let key_len =
            u32::from_le_bytes([byte_array[0], byte_array[1], byte_array[2], byte_array[3]])
                as usize;

        // - parse key data
        let byte_array = cursor.consume(key_len);
        if byte_array.is_err() {
            if byte_array.err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(kv_tupple_errors::from_bytes_parse_key_data_invalid_eof());
            } else {
                return Err(kv_tupple_errors::from_bytes_parse_key_data_invalid_bytes());
            }
        }
        let key_data = byte_array.unwrap();

        // - parse value length
        let byte_array = cursor.consume(4);
        if byte_array.is_err() {
            if byte_array.err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(kv_tupple_errors::from_bytes_parse_value_length_invalid_eof());
            } else {
                return Err(kv_tupple_errors::from_bytes_parse_value_length_invalid_bytes());
            }
        }
        let byte_array = byte_array.unwrap();
        let value_len =
            u32::from_le_bytes([byte_array[0], byte_array[1], byte_array[2], byte_array[3]])
                as usize;

        // - parse value data
        let byte_array = cursor.consume(value_len);
        if byte_array.is_err() {
            if byte_array.err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(kv_tupple_errors::from_bytes_parse_value_data_invalid_eof());
            } else {
                return Err(kv_tupple_errors::from_bytes_parse_value_data_invalid_bytes());
            }
        }
        let value_data = byte_array.unwrap();

        Ok(KVTupple {
            key: key_data.to_vec(),
            value: value_data.to_vec(),
        })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        KVTupple::from_byte_cursor(&mut Cursor::new(bytes))
    }

    pub fn key(&self) -> &[u8] {
        self.key.as_slice()
    }

    pub fn value(&self) -> &[u8] {
        self.value.as_slice()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // - key length
        bytes.extend_from_slice(&u32::to_le_bytes(self.key.len() as u32));
        // - key data
        bytes.extend_from_slice(self.key.as_slice());
        // - value length
        bytes.extend_from_slice(&u32::to_le_bytes(self.value.len() as u32));
        // - value data
        bytes.extend_from_slice(self.value.as_slice());
        bytes
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_kv_tupple_to_bytes() {
        let kv_tupple = KVTupple::new(
            &vec![0x10, 0x20, 0x30, 0x40],
            &vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x65],
        );
        let bytes = kv_tupple.to_bytes();
        assert_eq!(
            bytes,
            vec![
                0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
                0x10, 0x20, 0x30, 0x40, // key data
                0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
                0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
            ]
        );
    }

    #[test]
    fn test_bytes_to_kv_tupples() {
        let bytes = vec![
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x40, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
        ];
        let index_tupple_result = KVTupple::from_bytes(&bytes);
        assert_eq!(index_tupple_result.is_ok(), true);
        let index_tupple = index_tupple_result.unwrap();
        assert_eq!(index_tupple.key(), vec![0x10, 0x20, 0x30, 0x40]);
        assert_eq!(
            index_tupple.value(),
            vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x65]
        );
    }

    #[test]
    fn test_bytes_to_kv_and_back() {
        fn test(key: &[u8], value: &[u8]) {
            let kv_tupple_from_kv = KVTupple::new(key, value);
            let bytes = kv_tupple_from_kv.to_bytes();
            let kv_tupple_from_bytes = KVTupple::from_bytes(&bytes);
            assert_eq!(kv_tupple_from_bytes.is_ok(), true);
            let kv_tupple_from_bytes = kv_tupple_from_bytes.unwrap();
            assert_eq!(kv_tupple_from_bytes.key(), key.to_vec());
            assert_eq!(kv_tupple_from_bytes.value(), value.to_vec());
        }
        let key = vec![
            0x80, 0x70, 0x60, 0x50, 0x40, 0x30, 0x20, 0x10, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60,
            0x70, 0x80,
        ];
        let value = vec![
            0x85, 0x75, 0x65, 0x55, 0x45, 0x35, 0x25, 0x15, 0x15, 0x25, 0x35, 0x45, 0x55, 0x65,
            0x75, 0x85,
        ];
        test(&key, &value);
        let key = "some random key as string".as_bytes();
        let value = "some random value as string".as_bytes();
        test(&key, &value);
        let key = u32::to_le_bytes(u32::MAX);
        let value = u32::to_le_bytes(u32::MAX);
        test(&key, &value);
    }
}
