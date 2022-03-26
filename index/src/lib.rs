// use std::collections::{BTreeMap, HashMap, LinkedList};
// use util::byte_cursor::Cursor;
// use util::error::Error;

mod index_errors;

pub mod kv_tupple;

// pub fn kv_map_from_bytes(bytes: &[u8]) -> Result<LinkedList<kv_tupple::KVTupple>, Error> {
//     let mut cursor = Cursor::new(bytes);
//     let mut kv_map: LinkedList<kv_tupple::KVTupple> = LinkedList::new();
//     while cursor.remaining_bytes() > 0 {
//         let kv_tupple_result = kv_tupple::KVTupple::from_byte_cursor(&mut cursor);
//         if kv_tupple_result.is_err() {
//             return Err(kv_tupple_result.err().unwrap());
//         }
//         let kv_tupple = kv_tupple_result.unwrap();
//         kv_map.push_back(kv_tupple);
//     }
//     Ok(kv_map)
// }

// pub fn kv_map_to_bytes(kv_map: &LinkedList<kv_tupple::KVTupple>) -> Result<Vec<u8>, Error> {
//     let mut bytes: Vec<u8> = Vec::new();
//     for kv_tupple in kv_map {
//         let kv_tupple_bytes = kv_tupple.to_bytes();
//         bytes.extend(kv_tupple_bytes);
//     }
//     Ok(bytes)
// }

// pub type BTreeIndex = BTreeMap<Vec<u8>, Vec<u8>>;

// pub fn btree_index_from_bytes(bytes: &[u8]) -> Result<BTreeIndex, Error> {
//     let mut kv_map = kv_map_from_bytes(bytes)?;
//     let mut btree_index: BTreeIndex = BTreeMap::new();
//     while !kv_map.is_empty() {
//         let kv_tupple_result = kv_map.pop_front();
//         if kv_tupple_result.is_none() {
//             return Err(index_errors::btree_index_from_bytes_empty_tupple());
//         }
//         let kv_tupple = kv_tupple_result.unwrap();
//         if matches!(kv_tupple.index_crud(), kv_tupple::IndexCrud::INSERT) {
//             let key = kv_tupple.key();
//             let value = kv_tupple.value();
//             if key.is_none() || value.is_none() {
//                 // return Err(index_errors::btree_index_from_bytes_invalid_tupple());
//                 panic!("btree_index_from_bytes_invalid_tupple");
//             } else {
//                 btree_index.insert(key.unwrap(), value.unwrap());
//             }
//         } else if matches!(kv_tupple.index_crud(), kv_tupple::IndexCrud::DELETE) {
//             let key = kv_tupple.key();
//             if key.is_none() {
//                 panic!("btree_index_from_bytes_invalid_tupple");
//             } else {
//                 btree_index.remove(&key.unwrap());
//             }
//         }
//     }
//     Ok(btree_index)
// }

// pub fn btree_index_to_bytes(btree_index: &BTreeIndex) -> Result<Vec<u8>, Error> {
//     let mut kv_map: LinkedList<kv_tupple::KVTupple> = LinkedList::new();
//     for (key, value) in btree_index {
//         let kv_tupple = kv_tupple::KVTupple::new(kv_tupple::IndexCrud::INSERT, key, value);
//         kv_map.push_back(kv_tupple);
//     }
//     kv_map_to_bytes(&kv_map)
// }

// type HashIndex = HashMap<Vec<u8>, Vec<u8>>;

// pub fn hash_index_from_bytes(bytes: &[u8]) -> Result<HashIndex, Error> {
//     let mut kv_map = kv_map_from_bytes(bytes)?;
//     let mut hash_map: HashIndex = HashMap::new();
//     while !kv_map.is_empty() {
//         let kv_tupple_result = kv_map.pop_front();
//         if kv_tupple_result.is_none() {
//             return Err(index_errors::hash_index_from_bytes_empty_tupple());
//         }
//         let kv_tupple = kv_tupple_result.unwrap();
//         if matches!(kv_tupple.index_crud(), kv_tupple::IndexCrud::INSERT) {
//             let key = kv_tupple.key();
//             let value = kv_tupple.value();
//             if key.is_none() || value.is_none() {
//                 // return Err(index_errors::hash_index_from_bytes_invalid_tupple());
//                 panic!("hash_index_from_bytes_invalid_tupple");
//             } else {
//                 hash_map.insert(key.unwrap(), value.unwrap());
//             }
//         } else if matches!(kv_tupple.index_crud(), kv_tupple::IndexCrud::DELETE) {
//             let key = kv_tupple.key();
//             if key.is_none() {
//                 panic!("hash_index_from_bytes_invalid_tupple");
//             } else {
//                 hash_map.remove(&key.unwrap());
//             }
//         }
//     }
//     Ok(hash_map)
// }

// pub fn hash_index_to_bytes(hash_index: &HashIndex) -> Result<Vec<u8>, Error> {
//     let mut kv_map: LinkedList<kv_tupple::KVTupple> = LinkedList::new();
//     for (key, value) in hash_index {
//         let kv_tupple = kv_tupple::KVTupple::new(kv_tupple::IndexCrud::INSERT, key, value);
//         kv_map.push_back(kv_tupple);
//     }
//     kv_map_to_bytes(&kv_map)
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn kv_map_from_bytes_test() {
//         let bytes = vec![
//             0x00, // crud to insert
//             0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//             0x10, 0x20, 0x30, 0x40, // key data
//             0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//             0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
//             // ...
//             0x00, // crud to insert
//             0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//             0x10, 0x20, 0x30, 0x41, // key data
//             0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//             0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
//             // ...
//             0x00, // crud to insert
//             0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//             0x10, 0x20, 0x30, 0x42, // key data
//             0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//             0x15, 0x25, 0x35, 0x45, 0x55, 0x67, // value data
//             // ...
//             0x01, // crud to delete
//             0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//             0x10, 0x20, 0x30, 0x42, // key data
//             0x00, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//             // ...
//             0x00, // crud to insert
//             0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//             0x10, 0x20, 0x30, 0x42, // key data
//             0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//             0x15, 0x25, 0x35, 0x45, 0x55, 0x68, // value data
//         ];

//         let result = kv_map_from_bytes(&bytes);
//         assert!(result.is_ok());

//         let mut kv_map = result.unwrap();
//         assert_eq!(kv_map.len(), 5);

//         let kv_tupple = kv_map.pop_front().unwrap();
//         assert!(matches!(kv_tupple.index_crud(), kv_tupple::IndexCrud::INSERT));
//         assert_eq!(kv_tupple.key().unwrap(), [0x10, 0x20, 0x30, 0x40]);
//         assert_eq!(
//             kv_tupple.value().unwrap(),
//             [0x15, 0x25, 0x35, 0x45, 0x55, 0x65]
//         );

//         let kv_tupple = kv_map.pop_front().unwrap();
//         assert!(matches!(kv_tupple.index_crud(), kv_tupple::IndexCrud::INSERT));
//         assert_eq!(kv_tupple.key().unwrap(), [0x10, 0x20, 0x30, 0x41]);
//         assert_eq!(
//             kv_tupple.value().unwrap(),
//             [0x15, 0x25, 0x35, 0x45, 0x55, 0x66]
//         );

//         let kv_tupple = kv_map.pop_front().unwrap();
//         assert!(matches!(kv_tupple.index_crud(), kv_tupple::IndexCrud::INSERT));
//         assert_eq!(kv_tupple.key().unwrap(), [0x10, 0x20, 0x30, 0x42]);
//         assert_eq!(
//             kv_tupple.value().unwrap(),
//             [0x15, 0x25, 0x35, 0x45, 0x55, 0x67]
//         );

//         let kv_tupple = kv_map.pop_front().unwrap();
//         assert!(matches!(kv_tupple.index_crud(), kv_tupple::IndexCrud::DELETE));
//         assert_eq!(kv_tupple.key().unwrap(), [0x10, 0x20, 0x30, 0x42]);
//         assert_eq!(kv_tupple.value().unwrap(), []);

//         let kv_tupple = kv_map.pop_front().unwrap();
//         assert!(matches!(kv_tupple.index_crud(), kv_tupple::IndexCrud::INSERT));
//         assert_eq!(kv_tupple.key().unwrap(), [0x10, 0x20, 0x30, 0x42]);
//         assert_eq!(
//             kv_tupple.value().unwrap(),
//             [0x15, 0x25, 0x35, 0x45, 0x55, 0x68]
//         );
//     }

//     #[test]
//     fn kv_map_to_bytes_test() {
//         let mut kv_map: LinkedList<kv_tupple::KVTupple> = LinkedList::new();
//         let kv_tupple = kv_tupple::KVTupple::new(
//             kv_tupple::IndexCrud::INSERT,
//             &[0x10, 0x20, 0x30, 0x40],
//             &[0x15, 0x25, 0x35, 0x45, 0x55, 0x65],
//         );
//         kv_map.push_back(kv_tupple);
//         let kv_tupple = kv_tupple::KVTupple::new(
//             kv_tupple::IndexCrud::INSERT,
//             &[0x10, 0x20, 0x30, 0x41],
//             &[0x15, 0x25, 0x35, 0x45, 0x55, 0x66],
//         );
//         kv_map.push_back(kv_tupple);
//         let result = kv_map_to_bytes(&kv_map);
//         assert!(result.is_ok());
//         let bytes = result.unwrap();
//         assert_eq!(
//             bytes,
//             vec![
//                 0x00, // crud to insert
//                 0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//                 0x10, 0x20, 0x30, 0x40, // key data
//                 0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//                 0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
//                 // ...
//                 0x00, // crud to insert
//                 0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//                 0x10, 0x20, 0x30, 0x41, // key data
//                 0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//                 0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
//             ]
//         );
//     }

//     #[test]
//     fn btree_index_from_bytes_test() {
//         let bytes = vec![
//             0x00, // crud to insert
//             0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//             0x10, 0x20, 0x30, 0x40, // key data
//             0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//             0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
//             // ...
//             0x00, // crud to insert
//             0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//             0x10, 0x20, 0x30, 0x41, // key data
//             0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//             0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
//         ];
//         let result = btree_index_from_bytes(&bytes);
//         assert!(result.is_ok());
//         let btree_index = result.unwrap();
//         assert_eq!(btree_index.len(), 2);
//         assert_eq!(
//             btree_index.get(&vec![0x10, 0x20, 0x30, 0x40]),
//             Some(&vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x65])
//         );
//         assert_eq!(
//             btree_index.get(&vec![0x10, 0x20, 0x30, 0x41]),
//             Some(&vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x66])
//         );
//     }

//     #[test]
//     fn btree_index_to_bytes_test() {
//         let mut btree_index: BTreeIndex = BTreeMap::new();
//         btree_index.insert(
//             vec![0x10, 0x20, 0x30, 0x40],
//             vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x65],
//         );
//         btree_index.insert(
//             vec![0x10, 0x20, 0x30, 0x41],
//             vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x66],
//         );
//         let result = btree_index_to_bytes(&btree_index);
//         assert!(result.is_ok());
//         let bytes = result.unwrap();
//         assert_eq!(
//             bytes,
//             vec![
//                 0x00, // crud to insert
//                 0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//                 0x10, 0x20, 0x30, 0x40, // key data
//                 0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//                 0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
//                 // ...
//                 0x00, // crud to insert
//                 0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//                 0x10, 0x20, 0x30, 0x41, // key data
//                 0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//                 0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
//             ]
//         );
//     }

//     #[test]
//     fn btree_index_from_bytes_and_to_back() {
//         let mut btree_index: BTreeIndex = BTreeMap::new();
//         btree_index.insert("One".as_bytes().to_vec(), "एक".as_bytes().to_vec());
//         btree_index.insert("Two".as_bytes().to_vec(), "दो".as_bytes().to_vec());
//         btree_index.insert("Three".as_bytes().to_vec(), "तीन".as_bytes().to_vec());
//         btree_index.insert("Four".as_bytes().to_vec(), "चार".as_bytes().to_vec());
//         let btree_index = btree_index; // immutable
//         let bytes_result = btree_index_to_bytes(&btree_index);
//         assert!(bytes_result.is_ok());
//         let bytes = bytes_result.unwrap();

//         // parse bytes with btree_index_from_bytes
//         let result = btree_index_from_bytes(&bytes);
//         assert!(result.is_ok());
//         let parsed_btree_index = result.unwrap();
//         assert_eq!(parsed_btree_index, btree_index);
//     }

//     #[test]
//     fn hash_index_from_bytes_test() {
//         let bytes = vec![
//             0x00, // crud to insert
//             0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//             0x10, 0x20, 0x30, 0x40, // key data
//             0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//             0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
//             // ...
//             0x00, // crud to insert
//             0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//             0x10, 0x20, 0x30, 0x41, // key data
//             0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//             0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
//         ];
//         let result = hash_index_from_bytes(&bytes);
//         assert!(result.is_ok());
//         let hash_map = result.unwrap();
//         assert_eq!(hash_map.len(), 2);
//         assert_eq!(
//             hash_map.get(&vec![0x10, 0x20, 0x30, 0x40]),
//             Some(&vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x65])
//         );
//         assert_eq!(
//             hash_map.get(&vec![0x10, 0x20, 0x30, 0x41]),
//             Some(&vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x66])
//         );
//     }

//     #[test]
//     fn hash_index_to_bytes_test() {
//         let mut hash_map: HashIndex = HashMap::new();
//         hash_map.insert(
//             vec![0x10, 0x20, 0x30, 0x40],
//             vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x65],
//         );
//         hash_map.insert(
//             vec![0x10, 0x20, 0x30, 0x41],
//             vec![0x15, 0x25, 0x35, 0x45, 0x55, 0x66],
//         );
//         let result = hash_index_to_bytes(&hash_map);
//         assert!(result.is_ok());
//         let bytes = result.unwrap();
//         assert!(
//             (bytes
//                 == [
//                     0x00, // crud to insert
//                     0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//                     0x10, 0x20, 0x30, 0x40, // key data
//                     0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//                     0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
//                     // ...
//                     0x00, // crud to insert
//                     0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//                     0x10, 0x20, 0x30, 0x41, // key data
//                     0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//                     0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
//                 ])
//                 || (bytes
//                     == [
//                         0x00, // crud to insert
//                         0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//                         0x10, 0x20, 0x30, 0x41, // key data
//                         0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//                         0x15, 0x25, 0x35, 0x45, 0x55, 0x66, // value data
//                         // ...
//                         0x00, // crud to insert
//                         0x04, 0x00, 0x00, 0x00, // little endian 4 bytes key length
//                         0x10, 0x20, 0x30, 0x40, // key data
//                         0x06, 0x00, 0x00, 0x00, // little endian 4 bytes value length
//                         0x15, 0x25, 0x35, 0x45, 0x55, 0x65, // value data
//                     ])
//         );
//     }

//     #[test]
//     fn hash_index_from_bytes_and_to_back() {
//         let mut hash_index: HashIndex = HashMap::new();
//         hash_index.insert("Une".as_bytes().to_vec(), "一".as_bytes().to_vec());
//         hash_index.insert("Deux".as_bytes().to_vec(), "二".as_bytes().to_vec());
//         hash_index.insert("Trois".as_bytes().to_vec(), "三".as_bytes().to_vec());
//         hash_index.insert("Quatre".as_bytes().to_vec(), "四".as_bytes().to_vec());
//         let hash_index = hash_index; // immutable
//         let bytes_result = hash_index_to_bytes(&hash_index);
//         assert!(bytes_result.is_ok());
//         let bytes = bytes_result.unwrap();

//         // parse bytes with hash_index_from_bytes
//         let result = hash_index_from_bytes(&bytes);
//         assert!(result.is_ok());
//         let parsed_hash_map = result.unwrap();
//         assert_eq!(parsed_hash_map, hash_index);
//     }
// }
