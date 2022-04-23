use util::byte_cursor::Cursor;
use util::error::Error;

mod tuple_errors;

pub enum IndexCrud {
    DELETE, // delete the key
    INSERT, // insert new value to key
    REMOVE, // remove the value from key
    NONE,
}

impl Clone for IndexCrud {
    fn clone(&self) -> Self {
        match self {
            IndexCrud::DELETE => IndexCrud::DELETE,
            IndexCrud::INSERT => IndexCrud::INSERT,
            IndexCrud::REMOVE => IndexCrud::REMOVE,
            IndexCrud::NONE => IndexCrud::NONE,
        }
    }
}

impl IndexCrud {
    fn index_crud_from_byte(byte: u8) -> IndexCrud {
        match byte {
            0 => IndexCrud::DELETE,
            1 => IndexCrud::INSERT,
            2 => IndexCrud::REMOVE,
            _ => IndexCrud::NONE,
        }
    }

    fn index_crud_to_byte(&self) -> u8 {
        match self {
            IndexCrud::DELETE => 0,
            IndexCrud::INSERT => 1,
            IndexCrud::REMOVE => 2,
            IndexCrud::NONE => 3,
        }
    }
}

type KeyLength = u32;
pub type KeyData = Vec<u8>;
const KEY_LENGTH_SIZE: usize = std::mem::size_of::<KeyLength>();

type ValueLength = u32;
pub type ValueData = Vec<u8>;
const VALUE_LENGTH_SIZE: usize = std::mem::size_of::<ValueLength>();

pub struct KVTuple {
    index_crud: IndexCrud,
    key: Option<KeyData>,
    value: Option<ValueData>,
}

impl KVTuple {
    fn new(crud: IndexCrud, key: Option<&[u8]>, value: Option<&[u8]>) -> Self {
        KVTuple {
            index_crud: crud,
            key: key.map(|k| k.to_vec()),
            value: value.map(|v| v.to_vec()),
        }
    }

    pub fn new_delete(key: &[u8]) -> Self {
        KVTuple::new(IndexCrud::DELETE, Some(key), None)
    }

    pub fn new_insert(key: &[u8], value: &[u8]) -> Self {
        KVTuple::new(IndexCrud::INSERT, Some(key), Some(value))
    }

    pub fn new_remove(key: &[u8], value: &[u8]) -> Self {
        KVTuple::new(IndexCrud::REMOVE, Some(key), Some(value))
    }

    fn index_crud_from_cursor(cursor: &mut Cursor) -> Result<IndexCrud, Error> {
        let byte_array = cursor.consume(1);
        if byte_array.is_err() {
            if byte_array.err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(tuple_errors::index_crud_from_cursor_invalid_eof_at_crud());
            } else {
                return Err(tuple_errors::index_crud_from_cursor_invalid_bytes());
            }
        }
        let crud_byte = byte_array.unwrap()[0];
        let index_crud = IndexCrud::index_crud_from_byte(crud_byte);
        if matches!(index_crud, IndexCrud::NONE) {
            return Err(tuple_errors::index_crud_from_cursor_invalid_crud());
        }
        Ok(index_crud)
    }

    fn key_from_cursor(cursor: &mut Cursor) -> Result<Option<KeyData>, Error> {
        // parse key length
        let byte_array = cursor.consume(KEY_LENGTH_SIZE);
        if byte_array.is_err() {
            if byte_array.err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(tuple_errors::key_from_cursor_invalid_eof_at_key_len());
            } else {
                return Err(tuple_errors::key_from_cursor_invalid_bytes_at_key_len());
            }
        }
        let byte_array = byte_array.unwrap();
        let key_len =
            KeyLength::from_le_bytes([byte_array[0], byte_array[1], byte_array[2], byte_array[3]])
                as usize;

        // parse key data
        let byte_array = cursor.consume(key_len);
        if byte_array.is_err() {
            if byte_array.err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(tuple_errors::key_from_cursor_invalid_eof_at_key_data());
            } else {
                return Err(tuple_errors::key_from_cursor_invalid_bytes_at_key_data());
            }
        }
        let key_data = byte_array.unwrap();

        Ok(Some(key_data.to_vec()))
    }

    fn value_from_cursor(cursor: &mut Cursor) -> Result<ValueData, Error> {
        // parse value length
        let byte_array = cursor.consume(VALUE_LENGTH_SIZE);
        if byte_array.is_err() {
            if byte_array.err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(tuple_errors::value_from_cursor_invalid_eof_at_value_len());
            } else {
                return Err(tuple_errors::value_from_cursor_invalid_bytes_at_value_len());
            }
        }
        let byte_array = byte_array.unwrap();
        let value_len = ValueLength::from_le_bytes([
            byte_array[0],
            byte_array[1],
            byte_array[2],
            byte_array[3],
        ]) as usize;

        // parse value data
        let byte_array = cursor.consume(value_len);
        if byte_array.is_err() {
            if byte_array.err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(tuple_errors::value_from_cursor_invalid_eof_at_value_data());
            } else {
                return Err(tuple_errors::value_from_cursor_invalid_bytes_at_value_data());
            }
        }
        let value_data = byte_array.unwrap();

        Ok(value_data.to_vec())
    }

    pub fn from_byte_cursor(cursor: &mut Cursor) -> Result<Self, Error> {
        let index_crud = KVTuple::index_crud_from_cursor(cursor)?;
        match index_crud {
            IndexCrud::DELETE => {
                let key = KVTuple::key_from_cursor(cursor)?;
                Ok(KVTuple::new_delete(key.unwrap().as_slice()))
            }
            IndexCrud::INSERT => {
                let key = KVTuple::key_from_cursor(cursor)?;
                let value = KVTuple::value_from_cursor(cursor)?;
                Ok(KVTuple::new_insert(
                    key.unwrap().as_slice(),
                    value.as_slice(),
                ))
            }
            IndexCrud::REMOVE => {
                let key = KVTuple::key_from_cursor(cursor)?;
                let value = KVTuple::value_from_cursor(cursor)?;
                Ok(KVTuple::new_remove(
                    key.unwrap().as_slice(),
                    value.as_slice(),
                ))
            }
            IndexCrud::NONE => Err(tuple_errors::from_byte_cursor_invalid_crud()),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        KVTuple::from_byte_cursor(&mut Cursor::new(bytes))
    }

    pub fn index_crud(&self) -> IndexCrud {
        self.index_crud.clone()
    }

    pub fn key(&self) -> Option<KeyData> {
        match self.key {
            None => None,
            _ => Some(self.key.as_ref().unwrap().clone()),
        }
    }

    pub fn value(&self) -> Option<ValueData> {
        match self.value {
            None => None,
            _ => Some(self.value.as_ref().unwrap().clone()),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // - crud byte
        bytes.extend_from_slice(&[self.index_crud.index_crud_to_byte()]);

        // - key
        let key = self.key();
        if key.is_some() {
            let key = key.unwrap();
            bytes.extend_from_slice(&u32::to_le_bytes(key.len() as u32));
            bytes.extend_from_slice(&key);
        } else {
            return bytes;
        }

        // - value
        if let Some(value) = self.value() {
            bytes.extend_from_slice(&u32::to_le_bytes(value.len() as u32));
            bytes.extend_from_slice(&value);
        }

        bytes
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_kv_tuple_to_bytes() {
        let kv_tuple = KVTuple::new_delete(&[0x10, 0x20, 0x30, 0x40]);
        let bytes = kv_tuple.to_bytes();
        assert_eq!(
            bytes,
            vec![
                0x00, // crud to delete
                0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
                0x10, 0x20, 0x30, 0x40, // key data
            ]
        );

        let kv_tuple = KVTuple::new_insert(
            &[0x10, 0x20, 0x30, 0x40],
            &[0x15, 0x25, 0x35, 0x45, 0x55, 0x65],
        );
        let bytes = kv_tuple.to_bytes();
        assert_eq!(
            bytes,
            vec![
                0x01, // crud to insert
                0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
                0x10, 0x20, 0x30, 0x40, // key data
                0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
                0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
            ]
        );

        let kv_tuple = KVTuple::new_remove(
            &[0x10, 0x20, 0x30, 0x40],
            &[0x15, 0x25, 0x35, 0x45, 0x55, 0x65],
        );
        let bytes = kv_tuple.to_bytes();
        assert_eq!(
            bytes,
            vec![
                0x02, // crud to remove
                0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
                0x10, 0x20, 0x30, 0x40, // key data
                0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
                0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
            ]
        );
    }

    #[test]
    fn test_bytes_to_kv_tuples() {
        let bytes = vec![
            0x00, // crud to delete
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x40, // key data
        ];
        let kv_tuple = KVTuple::from_bytes(&bytes).unwrap();
        assert!(matches!(kv_tuple.index_crud(), IndexCrud::DELETE));
        assert_eq!(kv_tuple.key(), Some(vec![0x10, 0x20, 0x30, 0x40]));
        assert_eq!(kv_tuple.value(), None);

        let bytes = vec![
            0x01, // crud to insert
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x40, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
        ];
        let kv_tuple = KVTuple::from_bytes(&bytes).unwrap();
        assert!(matches!(kv_tuple.index_crud(), IndexCrud::INSERT));
        assert_eq!(kv_tuple.key(), Some(vec![0x10, 0x20, 0x30, 0x40]));
        assert_eq!(
            kv_tuple.value(),
            Some(vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x65])
        );

        let bytes = vec![
            0x02, // crud to remove
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x40, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
        ];
        let kv_tuple = KVTuple::from_bytes(&bytes).unwrap();
        assert!(matches!(kv_tuple.index_crud(), IndexCrud::REMOVE));
        assert_eq!(kv_tuple.key(), Some(vec![0x10, 0x20, 0x30, 0x40]));
        assert_eq!(
            kv_tuple.value(),
            Some(vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x65])
        );
    }

    #[test]
    fn test_bytes_to_kv_and_back() {
        fn test_delete(key: &[u8]) {
            let kv_tuple_from_kv = KVTuple::new_delete(key);
            let bytes = kv_tuple_from_kv.to_bytes();
            let kv_tuple_from_bytes = KVTuple::from_bytes(&bytes);
            assert_eq!(kv_tuple_from_bytes.is_ok(), true);
            let kv_tuple_from_bytes = kv_tuple_from_bytes.unwrap();
            assert_eq!(kv_tuple_from_bytes.key(), Some(key.to_vec()));
            assert_eq!(kv_tuple_from_bytes.value(), None);
        }
        fn test_insert(key: &[u8], value: &[u8]) {
            let kv_tuple_from_kv = KVTuple::new_insert(key, value);
            let bytes = kv_tuple_from_kv.to_bytes();
            let kv_tuple_from_bytes = KVTuple::from_bytes(&bytes);
            assert_eq!(kv_tuple_from_bytes.is_ok(), true);
            let kv_tuple_from_bytes = kv_tuple_from_bytes.unwrap();
            assert_eq!(kv_tuple_from_bytes.key(), Some(key.to_vec()));
            assert_eq!(kv_tuple_from_bytes.value(), Some(value.to_vec()));
        }
        fn test_remove(key: &[u8], value: &[u8]) {
            let kv_tuple_from_kv = KVTuple::new_remove(key, value);
            let bytes = kv_tuple_from_kv.to_bytes();
            let kv_tuple_from_bytes = KVTuple::from_bytes(&bytes);
            assert_eq!(kv_tuple_from_bytes.is_ok(), true);
            let kv_tuple_from_bytes = kv_tuple_from_bytes.unwrap();
            assert_eq!(kv_tuple_from_bytes.key(), Some(key.to_vec()));
            assert_eq!(kv_tuple_from_bytes.value(), Some(value.to_vec()));
        }
        let key = vec![
            0x80, 0x70, 0x60, 0x50, 0x40, 0x30, 0x20, 0x10, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60,
            0x70, 0x80,
        ];
        let value = vec![
            0x85, 0x75, 0x65, 0x55, 0x45, 0x35, 0x25, 0x15, 0x15, 0x25, 0x35, 0x45, 0x55, 0x65,
            0x75, 0x85,
        ];
        test_delete(&key);
        test_insert(&key, &value);
        test_remove(&key, &value);
        let key = "some random key as string".as_bytes();
        let value = "some random value as string".as_bytes();
        test_delete(key);
        test_insert(key, value);
        test_remove(key, value);
        let key = u32::to_le_bytes(u32::MAX);
        let value = u32::to_le_bytes(u32::MAX);
        test_delete(&key);
        test_insert(&key, &value);
        test_remove(&key, &value);
        let key = u32::to_le_bytes(u32::MAX);
        let value = u32::to_le_bytes(u32::MAX);
        test_delete(&key);
        test_insert(&key, &value);
        test_remove(&key, &value);
        let key = vec![];
        let value = vec![];
        test_delete(&key);
        test_insert(&key, &value);
        test_remove(&key, &value);
    }
}
