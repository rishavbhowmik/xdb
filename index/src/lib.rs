use std::collections::{BTreeMap, HashMap, LinkedList};
use util::byte_cursor::Cursor;
use util::error::Error;

pub mod index_errors;

pub mod kv_tupple;

pub fn kv_map_from_bytes(bytes: &[u8]) -> Result<LinkedList<kv_tupple::KVTupple>, Error> {
    let mut cursor = Cursor::new(bytes);
    let mut kv_map: LinkedList<kv_tupple::KVTupple> = LinkedList::new();
    while cursor.remaining_bytes() > 0 {
        let kv_tupple_result = kv_tupple::KVTupple::from_byte_cursor(&mut cursor);
        if kv_tupple_result.is_err() {
            return Err(kv_tupple_result.err().unwrap());
        }
        let kv_tupple = kv_tupple_result.unwrap();
        kv_map.push_back(kv_tupple);
    }
    Ok(kv_map)
}

pub fn btree_index_from_bytes(bytes: &[u8]) -> Result<BTreeMap<Vec<u8>, Vec<u8>>, Error> {
    let mut kv_map = kv_map_from_bytes(bytes)?;
    let mut btree_index: BTreeMap<Vec<u8>, Vec<u8>> = BTreeMap::new();
    while !kv_map.is_empty() {
        let kv_tupple_result = kv_map.pop_front();
        if kv_tupple_result.is_none() {
            return Err(index_errors::btree_index_from_bytes_empty_tupple());
        }
        let kv_tupple = kv_tupple_result.unwrap();
        btree_index.insert(kv_tupple.key().to_vec(), kv_tupple.value().to_vec());
    }
    Ok(btree_index)
}

pub fn hash_map_from_bytes(bytes: &[u8]) -> Result<HashMap<Vec<u8>, Vec<u8>>, Error> {
    let mut kv_map = kv_map_from_bytes(bytes)?;
    let mut hash_map: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
    while !kv_map.is_empty() {
        let kv_tupple_result = kv_map.pop_front();
        if kv_tupple_result.is_none() {
            return Err(index_errors::hash_map_from_bytes_empty_tupple());
        }
        let kv_tupple = kv_tupple_result.unwrap();
        hash_map.insert(kv_tupple.key().to_vec(), kv_tupple.value().to_vec());
    }
    Ok(hash_map)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn kv_map_from_bytes_test() {
        let bytes = vec![
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x40, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
            // ...
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x41, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
        ];
        let result = kv_map_from_bytes(&bytes);
        assert!(result.is_ok());
        let mut kv_map = result.unwrap();
        assert_eq!(kv_map.len(), 2);
        let kv_tupple = kv_map.pop_front().unwrap();
        assert_eq!(kv_tupple.key(), &[0x10, 0x20, 0x30, 0x40]);
        assert_eq!(kv_tupple.value(), &[0x15, 0x25, 0x35, 0x45, 0x55, 0x65]);
        let kv_tupple = kv_map.pop_front().unwrap();
        assert_eq!(kv_tupple.key(), &[0x10, 0x20, 0x30, 0x41]);
        assert_eq!(kv_tupple.value(), &[0x15, 0x25, 0x35, 0x45, 0x55, 0x66]);
    }

    #[test]
    fn btree_index_from_bytes_test() {
        let bytes = vec![
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x40, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
            // ...
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x41, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
        ];
        let result = btree_index_from_bytes(&bytes);
        assert!(result.is_ok());
        let btree_index = result.unwrap();
        assert_eq!(btree_index.len(), 2);
        assert_eq!(
            btree_index.get(&vec![0x10, 0x20, 0x30, 0x40]),
            Some(&vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x65])
        );
        assert_eq!(
            btree_index.get(&vec![0x10, 0x20, 0x30, 0x41]),
            Some(&vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x66])
        );
    }

    #[test]
    fn hash_map_from_bytes_test() {
        let bytes = vec![
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x40, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
            // ...
            0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
            0x10, 0x20, 0x30, 0x41, // key data
            0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
            0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
        ];
        let result = hash_map_from_bytes(&bytes);
        assert!(result.is_ok());
        let hash_map = result.unwrap();
        assert_eq!(hash_map.len(), 2);
        assert_eq!(
            hash_map.get(&vec![0x10, 0x20, 0x30, 0x40]),
            Some(&vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x65])
        );
        assert_eq!(
            hash_map.get(&vec![0x10, 0x20, 0x30, 0x41]),
            Some(&vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x66])
        );
    }
}
