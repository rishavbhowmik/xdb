use util::byte_cursor::Cursor;
use util::error::Error;

pub mod tuple;

use std::collections::LinkedList;

pub fn kv_tuples_from_bytes(bytes: &[u8]) -> Result<LinkedList<tuple::KVTuple>, Error> {
    let mut cursor = Cursor::new(bytes);
    let mut kv_map: LinkedList<tuple::KVTuple> = LinkedList::new();
    while cursor.remaining_bytes() > 0 {
        let kv_tuple = tuple::KVTuple::from_byte_cursor(&mut cursor)?;
        kv_map.push_back(kv_tuple);
    }
    Ok(kv_map)
}

pub fn kv_tuples_to_bytes(kv_map: &LinkedList<tuple::KVTuple>) -> Result<Vec<u8>, Error> {
    let mut bytes: Vec<u8> = Vec::new();
    for kv_tuple in kv_map {
        let kv_tuple_bytes = kv_tuple.to_bytes();
        bytes.extend(kv_tuple_bytes);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn sample_kv_bytes() -> Vec<u8> {
        let sample: Vec<u8> = vec![
            0x01, // crud to insert
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x40, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
            // ...
            0x01, // crud to insert
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x41, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
            // ...
            0x01, // crud to insert
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x42, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x67, // value data
            // ...
            0x00, // crud to delete
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x42, // key data
            // ...
            0x01, // crud to insert
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x42, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x68, // value data
            // ...
            0x01, // crud to insert
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x42, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x16, 0x26, 0x36, 0x46, 0x56, 0x69, // value data
            // ...
            0x01, // crud to insert
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x42, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x17, 0x27, 0x37, 0x47, 0x57, 0x70, // value data
            // ...
            0x02, // crud to remove value
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x42, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x16, 0x26, 0x36, 0x46, 0x56, 0x69, // value data
            // ...
            0x02, // crud to remove value
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x42, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x17, 0x27, 0x37, 0x47, 0x57, 0x70, // value data
        ];
        sample
    }

    #[test]
    fn kv_map_from_bytes_test() {
        let bytes = sample_kv_bytes();

        let result = kv_tuples_from_bytes(&bytes);
        assert!(result.is_ok());

        let mut kv_map = result.unwrap();
        assert_eq!(kv_map.len(), 9);

        let kv_tuple = kv_map.pop_front().unwrap();
        assert!(matches!(kv_tuple.index_crud(), tuple::IndexCrud::INSERT));
        assert_eq!(kv_tuple.key().unwrap(), [0x10, 0x20, 0x30, 0x40]);
        assert_eq!(
            kv_tuple.value().unwrap(),
            [0x15, 0x25, 0x35, 0x45, 0x55, 0x65]
        );

        let kv_tuple = kv_map.pop_front().unwrap();
        assert!(matches!(kv_tuple.index_crud(), tuple::IndexCrud::INSERT));
        assert_eq!(kv_tuple.key().unwrap(), [0x10, 0x20, 0x30, 0x41]);
        assert_eq!(
            kv_tuple.value().unwrap(),
            [0x15, 0x25, 0x35, 0x45, 0x55, 0x66]
        );

        let kv_tuple = kv_map.pop_front().unwrap();
        assert!(matches!(kv_tuple.index_crud(), tuple::IndexCrud::INSERT));
        assert_eq!(kv_tuple.key().unwrap(), [0x10, 0x20, 0x30, 0x42]);
        assert_eq!(
            kv_tuple.value().unwrap(),
            [0x15, 0x25, 0x35, 0x45, 0x55, 0x67]
        );

        let kv_tuple = kv_map.pop_front().unwrap();
        assert!(matches!(kv_tuple.index_crud(), tuple::IndexCrud::DELETE));
        assert_eq!(kv_tuple.key().unwrap(), [0x10, 0x20, 0x30, 0x42]);
        assert!(kv_tuple.value().is_none());

        let kv_tuple = kv_map.pop_front().unwrap();
        assert!(matches!(kv_tuple.index_crud(), tuple::IndexCrud::INSERT));
        assert_eq!(kv_tuple.key().unwrap(), [0x10, 0x20, 0x30, 0x42]);
        assert_eq!(
            kv_tuple.value().unwrap(),
            [0x15, 0x25, 0x35, 0x45, 0x55, 0x68]
        );

        let kv_tuple = kv_map.pop_front().unwrap();
        assert!(matches!(kv_tuple.index_crud(), tuple::IndexCrud::INSERT));
        assert_eq!(kv_tuple.key().unwrap(), [0x10, 0x20, 0x30, 0x42]);
        assert_eq!(
            kv_tuple.value().unwrap(),
            [0x16, 0x26, 0x36, 0x46, 0x56, 0x69]
        );

        let kv_tuple = kv_map.pop_front().unwrap();
        assert!(matches!(kv_tuple.index_crud(), tuple::IndexCrud::INSERT));
        assert_eq!(kv_tuple.key().unwrap(), [0x10, 0x20, 0x30, 0x42]);
        assert_eq!(
            kv_tuple.value().unwrap(),
            [0x17, 0x27, 0x37, 0x47, 0x57, 0x70]
        );

        let kv_tuple = kv_map.pop_front().unwrap();
        assert!(matches!(kv_tuple.index_crud(), tuple::IndexCrud::REMOVE));
        assert_eq!(kv_tuple.key().unwrap(), [0x10, 0x20, 0x30, 0x42]);
        assert_eq!(
            kv_tuple.value().unwrap(),
            [0x16, 0x26, 0x36, 0x46, 0x56, 0x69]
        );

        let kv_tuple = kv_map.pop_front().unwrap();
        assert!(matches!(kv_tuple.index_crud(), tuple::IndexCrud::REMOVE));
        assert_eq!(kv_tuple.key().unwrap(), [0x10, 0x20, 0x30, 0x42]);
        assert_eq!(
            kv_tuple.value().unwrap(),
            [0x17, 0x27, 0x37, 0x47, 0x57, 0x70]
        );
    }

    #[test]
    fn kv_map_from_bytes_to_bytes_test() {
        let bytes = sample_kv_bytes();
        let kv_list = kv_tuples_from_bytes(&bytes).unwrap();
        let kv_bytes = kv_tuples_to_bytes(&kv_list).unwrap();
        assert_eq!(kv_bytes, bytes);
    }
}
