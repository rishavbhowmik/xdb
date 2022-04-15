use index::index_traits::{IndexCloneTrait, IndexSerializationTrait, IndexTrait, UniqueIndexTrait};
use index::{BTreeIndex, HashMapIndex, UniqueBTreeIndex, UniqueHashMapIndex};

fn read_full_file(file_name: &str) -> Vec<u8> {
    use std::fs::read;
    use std::path::Path;
    let read_result = read(Path::new(file_name));
    match read_result {
        Ok(data) => data,
        Err(e) => panic!("{:?}", e),
    }
}

fn remove_dir_contents(path: std::path::PathBuf) {
    use std::fs::{read_dir, remove_dir, remove_file};
    let path_copy = path.clone();
    for entry in read_dir(path_copy).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.file_type().unwrap().is_dir() {
            let path_copy = path.clone();
            remove_dir_contents(path_copy);
            let path_copy = path.clone();
            remove_dir(path_copy).unwrap();
        } else {
            remove_file(path).unwrap();
        }
    }
    let path_copy = path.clone();
    remove_dir(path_copy).unwrap();
}

// ... ... ... ... ... ... BTreeIndex ... ... ... ... ... ... ...

#[test]
fn btree_index_sync_new_log_file() {
    fn insert_tuple_test(
        btree_index: &mut BTreeIndex,
        tmp_file_path: &str,
        key: &str,
        value: &str,
    ) {
        // add to index in memory
        let result = btree_index.insert(key.as_bytes().to_vec(), value.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let btree_index = btree_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&sync_bytes).unwrap();
        // read from log file
        let bytes_in_log_file = read_full_file(tmp_file_path);
        let btree_index_from_file = BTreeIndex::from_bytes(&bytes_in_log_file);
        assert!(btree_index_from_file.is_ok());
        // verify if log_file is correct
        let btree_index_from_file = btree_index_from_file.unwrap();
        assert_eq!(
            btree_index.index_clone(),
            btree_index_from_file.index_clone()
        );
    }
    fn remove_tuple_test(
        btree_index: &mut BTreeIndex,
        tmp_file_path: &str,
        key: &str,
        value: &str,
    ) {
        let result = btree_index.remove(key.as_bytes().to_vec(), value.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let btree_index = btree_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&sync_bytes).unwrap();
        // read from log file
        let bytes_in_log_file = read_full_file(tmp_file_path);
        let btree_index_from_file = BTreeIndex::from_bytes(&bytes_in_log_file);
        assert!(btree_index_from_file.is_ok());
        // verify if log_file is correct
        let btree_index_from_file = btree_index_from_file.unwrap();
        assert_eq!(
            btree_index.index_clone(),
            btree_index_from_file.index_clone()
        );
    }
    fn delete_tuple_with_key_test(btree_index: &mut BTreeIndex, tmp_file_path: &str, key: &str) {
        let result = btree_index.delete(key.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let btree_index = btree_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&sync_bytes).unwrap();
        // read from log file
        let bytes_in_log_file = read_full_file(tmp_file_path);
        let btree_index_from_file = BTreeIndex::from_bytes(&bytes_in_log_file);
        assert!(btree_index_from_file.is_ok());
        // verify if log_file is correct
        let btree_index_from_file = btree_index_from_file.unwrap();
        assert_eq!(
            btree_index.index_clone(),
            btree_index_from_file.index_clone()
        );
    }

    // let tmp_file_path = "./tmp/btree_index_sync_new_log_file.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("btree_index_sync_new_log_file.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();

    // common index
    let mut btree_index = BTreeIndex::from_bytes(&[]).unwrap();

    insert_tuple_test(&mut btree_index, tmp_file_path, "1x1", "1");
    insert_tuple_test(&mut btree_index, tmp_file_path, "2x2", "4");
    insert_tuple_test(&mut btree_index, tmp_file_path, "3x3", "9");
    insert_tuple_test(&mut btree_index, tmp_file_path, "4x4", "16");
    insert_tuple_test(&mut btree_index, tmp_file_path, "5x5", "25");
    insert_tuple_test(&mut btree_index, tmp_file_path, "6x6", "36");
    insert_tuple_test(&mut btree_index, tmp_file_path, "7x7", "49");
    insert_tuple_test(&mut btree_index, tmp_file_path, "8x8", "64");
    insert_tuple_test(&mut btree_index, tmp_file_path, "9x9", "81");
    insert_tuple_test(&mut btree_index, tmp_file_path, "10x10", "100");
    insert_tuple_test(&mut btree_index, tmp_file_path, "1x1", "One");
    insert_tuple_test(&mut btree_index, tmp_file_path, "2x2", "Four");
    insert_tuple_test(&mut btree_index, tmp_file_path, "3x3", "Nine");
    insert_tuple_test(&mut btree_index, tmp_file_path, "4x4", "Sixteen");
    insert_tuple_test(&mut btree_index, tmp_file_path, "5x5", "Twenty-five");
    insert_tuple_test(&mut btree_index, tmp_file_path, "6x6", "Thirty-six");
    insert_tuple_test(&mut btree_index, tmp_file_path, "7x7", "Forty-nine");
    insert_tuple_test(&mut btree_index, tmp_file_path, "8x8", "Sixty-four");
    insert_tuple_test(&mut btree_index, tmp_file_path, "9x9", "Eighty-one");
    insert_tuple_test(&mut btree_index, tmp_file_path, "10x10", "One-hundred");

    remove_tuple_test(&mut btree_index, tmp_file_path, "10x10", "100");
    remove_tuple_test(&mut btree_index, tmp_file_path, "9x9", "81");
    remove_tuple_test(&mut btree_index, tmp_file_path, "8x8", "64");
    remove_tuple_test(&mut btree_index, tmp_file_path, "7x7", "49");
    remove_tuple_test(&mut btree_index, tmp_file_path, "6x6", "36");
    remove_tuple_test(&mut btree_index, tmp_file_path, "5x5", "25");
    remove_tuple_test(&mut btree_index, tmp_file_path, "4x4", "16");
    remove_tuple_test(&mut btree_index, tmp_file_path, "3x3", "9");
    remove_tuple_test(&mut btree_index, tmp_file_path, "2x2", "4");
    remove_tuple_test(&mut btree_index, tmp_file_path, "2x2", "Four");

    delete_tuple_with_key_test(&mut btree_index, tmp_file_path, "1x1");
    // 2x2 key is already deleted as all its values are deleted
    delete_tuple_with_key_test(&mut btree_index, tmp_file_path, "3x3");
    delete_tuple_with_key_test(&mut btree_index, tmp_file_path, "4x4");
    delete_tuple_with_key_test(&mut btree_index, tmp_file_path, "5x5");
    delete_tuple_with_key_test(&mut btree_index, tmp_file_path, "6x6");
    delete_tuple_with_key_test(&mut btree_index, tmp_file_path, "7x7");
    delete_tuple_with_key_test(&mut btree_index, tmp_file_path, "8x8");
    delete_tuple_with_key_test(&mut btree_index, tmp_file_path, "9x9");
    delete_tuple_with_key_test(&mut btree_index, tmp_file_path, "10x10");

    // clean up
    remove_dir_contents(tmp_dir_path);
}

fn btree_index_sync_existing_log_file() {}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

// ... ... ... ... ... ... UniqueBTreeIndex ... ... ... ... ... .

#[test]
fn unqiue_btree_index_sync_new_log_file() {
    fn insert_tuple_test(
        btree_index: &mut UniqueBTreeIndex,
        tmp_file_path: &str,
        key: &str,
        value: &str,
    ) {
        // add to index in memory
        let result = btree_index.set(key.as_bytes().to_vec(), value.as_bytes().to_vec(), false);
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let btree_index = btree_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&sync_bytes).unwrap();
        // read from log file
        let bytes_in_log_file = read_full_file(tmp_file_path);
        let btree_index_from_file = UniqueBTreeIndex::from_bytes(&bytes_in_log_file);
        assert!(btree_index_from_file.is_ok());
        // verify if log_file is correct
        let btree_index_from_file = btree_index_from_file.unwrap();
        assert_eq!(
            btree_index.index_clone(),
            btree_index_from_file.index_clone()
        );
    }

    fn delete_tuple_test(btree_index: &mut UniqueBTreeIndex, tmp_file_path: &str, key: &str) {
        // update index in memory
        let result = btree_index.delete(key.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let btree_index = btree_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&sync_bytes).unwrap();
        // read from log file
        let bytes_in_log_file = read_full_file(tmp_file_path);
        let btree_index_from_file = UniqueBTreeIndex::from_bytes(&bytes_in_log_file);
        assert!(btree_index_from_file.is_ok());
        // verify if log_file is correct
        let btree_index_from_file = btree_index_from_file.unwrap();
        assert_eq!(
            btree_index.index_clone(),
            btree_index_from_file.index_clone()
        );
    }

    // let tmp_file_path = "./tmp/unqiue_btree_index_sync_new_log_file.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("unqiue_btree_index_sync_new_log_file.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();

    // common index
    let mut btree_index = UniqueBTreeIndex::from_bytes(&[]).unwrap();

    insert_tuple_test(&mut btree_index, tmp_file_path, "1x1", "1");
    insert_tuple_test(&mut btree_index, tmp_file_path, "2x2", "4");
    insert_tuple_test(&mut btree_index, tmp_file_path, "3x3", "9");
    insert_tuple_test(&mut btree_index, tmp_file_path, "4x4", "16");
    insert_tuple_test(&mut btree_index, tmp_file_path, "5x5", "25");
    insert_tuple_test(&mut btree_index, tmp_file_path, "6x6", "36");
    insert_tuple_test(&mut btree_index, tmp_file_path, "7x7", "49");
    insert_tuple_test(&mut btree_index, tmp_file_path, "8x8", "64");
    insert_tuple_test(&mut btree_index, tmp_file_path, "9x9", "81");
    insert_tuple_test(&mut btree_index, tmp_file_path, "10x10", "100");

    delete_tuple_test(&mut btree_index, tmp_file_path, "1x1");
    delete_tuple_test(&mut btree_index, tmp_file_path, "2x2");
    delete_tuple_test(&mut btree_index, tmp_file_path, "3x3");
    delete_tuple_test(&mut btree_index, tmp_file_path, "4x4");
    delete_tuple_test(&mut btree_index, tmp_file_path, "5x5");
    delete_tuple_test(&mut btree_index, tmp_file_path, "6x6");
    delete_tuple_test(&mut btree_index, tmp_file_path, "7x7");
    delete_tuple_test(&mut btree_index, tmp_file_path, "8x8");
    delete_tuple_test(&mut btree_index, tmp_file_path, "9x9");
    delete_tuple_test(&mut btree_index, tmp_file_path, "10x10");

    // clean up
    remove_dir_contents(tmp_dir_path);
}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

// ... ... ... ... ... ... HashMapIndex ... ... ... ... ... ... .

#[test]
fn hash_map_index_new_log_file() {
    fn insert_tuple_test(
        hash_map_index: &mut HashMapIndex,
        tmp_file_path: &str,
        key: &str,
        value: &str,
    ) {
        // add to index in memory
        let result = hash_map_index.insert(key.as_bytes().to_vec(), value.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let hash_map_index = hash_map_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&sync_bytes).unwrap();
        // read from log file
        let bytes_in_log_file = read_full_file(tmp_file_path);
        let hash_map_index_from_file = HashMapIndex::from_bytes(&bytes_in_log_file);
        assert!(hash_map_index_from_file.is_ok());
        // verify if log_file is correct
        let hash_map_index_from_file = hash_map_index_from_file.unwrap();
        assert_eq!(
            hash_map_index.index_clone(),
            hash_map_index_from_file.index_clone()
        );
    }
    fn remove_tuple_test(
        hash_map_index: &mut HashMapIndex,
        tmp_file_path: &str,
        key: &str,
        value: &str,
    ) {
        let result = hash_map_index.remove(key.as_bytes().to_vec(), value.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let hash_map_index = hash_map_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&sync_bytes).unwrap();
        // read from log file
        let bytes_in_log_file = read_full_file(tmp_file_path);
        let hash_map_index_from_file = HashMapIndex::from_bytes(&bytes_in_log_file);
        assert!(hash_map_index_from_file.is_ok());
        // verify if log_file is correct
        let hash_map_index_from_file = hash_map_index_from_file.unwrap();
        assert_eq!(
            hash_map_index.index_clone(),
            hash_map_index_from_file.index_clone()
        );
    }
    fn delete_tuple_with_key_test(
        hash_map_index: &mut HashMapIndex,
        tmp_file_path: &str,
        key: &str,
    ) {
        let result = hash_map_index.delete(key.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let hash_map_index = hash_map_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&sync_bytes).unwrap();
        // read from log file
        let bytes_in_log_file = read_full_file(tmp_file_path);
        let hash_map_index_from_file = HashMapIndex::from_bytes(&bytes_in_log_file);
        assert!(hash_map_index_from_file.is_ok());
        // verify if log_file is correct
        let hash_map_index_from_file = hash_map_index_from_file.unwrap();
        assert_eq!(
            hash_map_index.index_clone(),
            hash_map_index_from_file.index_clone()
        );
    }

    // let tmp_file_path = "./tmp/hash_map_index_test.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("hash_map_index_test.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();

    // common index
    let mut hash_map_index = HashMapIndex::from_bytes(&[]).unwrap();

    insert_tuple_test(&mut hash_map_index, tmp_file_path, "1x1", "1");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "2x2", "4");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "3x3", "9");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "4x4", "16");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "5x5", "25");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "6x6", "36");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "7x7", "49");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "8x8", "64");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "9x9", "81");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "10x10", "100");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "1x1", "One");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "2x2", "Four");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "3x3", "Nine");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "4x4", "Sixteen");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "5x5", "Twenty-five");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "6x6", "Thirty-six");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "7x7", "Forty-nine");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "8x8", "Sixty-four");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "9x9", "Eighty-one");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "10x10", "One-hundred");

    remove_tuple_test(&mut hash_map_index, tmp_file_path, "10x10", "100");
    remove_tuple_test(&mut hash_map_index, tmp_file_path, "9x9", "81");
    remove_tuple_test(&mut hash_map_index, tmp_file_path, "8x8", "64");
    remove_tuple_test(&mut hash_map_index, tmp_file_path, "7x7", "49");
    remove_tuple_test(&mut hash_map_index, tmp_file_path, "6x6", "36");
    remove_tuple_test(&mut hash_map_index, tmp_file_path, "5x5", "25");
    remove_tuple_test(&mut hash_map_index, tmp_file_path, "4x4", "16");
    remove_tuple_test(&mut hash_map_index, tmp_file_path, "3x3", "9");
    remove_tuple_test(&mut hash_map_index, tmp_file_path, "2x2", "4");
    remove_tuple_test(&mut hash_map_index, tmp_file_path, "2x2", "Four");

    delete_tuple_with_key_test(&mut hash_map_index, tmp_file_path, "1x1");
    // 2x2 key is already deleted as all its values are deleted
    delete_tuple_with_key_test(&mut hash_map_index, tmp_file_path, "3x3");
    delete_tuple_with_key_test(&mut hash_map_index, tmp_file_path, "4x4");
    delete_tuple_with_key_test(&mut hash_map_index, tmp_file_path, "5x5");
    delete_tuple_with_key_test(&mut hash_map_index, tmp_file_path, "6x6");
    delete_tuple_with_key_test(&mut hash_map_index, tmp_file_path, "7x7");
    delete_tuple_with_key_test(&mut hash_map_index, tmp_file_path, "8x8");
    delete_tuple_with_key_test(&mut hash_map_index, tmp_file_path, "9x9");
    delete_tuple_with_key_test(&mut hash_map_index, tmp_file_path, "10x10");

    // clean up
    remove_dir_contents(tmp_dir_path);
}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

// ... ... ... ... ... ... UniqueHashMapIndex ... ... ... ... ...

#[test]
fn unique_hash_map_index_new_log_file() {
    fn insert_tuple_test(
        hash_map_index: &mut UniqueHashMapIndex,
        tmp_file_path: &str,
        key: &str,
        value: &str,
    ) {
        // add to index in memory
        let result = hash_map_index.set(key.as_bytes().to_vec(), value.as_bytes().to_vec(), false);
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let hash_map_index = hash_map_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&sync_bytes).unwrap();
        // read from log file
        let bytes_in_log_file = read_full_file(tmp_file_path);
        let hash_map_index_from_file = UniqueHashMapIndex::from_bytes(&bytes_in_log_file);
        assert!(hash_map_index_from_file.is_ok());
        // verify if log_file is correct
        let hash_map_index_from_file = hash_map_index_from_file.unwrap();
        assert_eq!(
            hash_map_index.index_clone(),
            hash_map_index_from_file.index_clone()
        );
    }

    fn delete_tuple_test(hash_map_index: &mut UniqueHashMapIndex, tmp_file_path: &str, key: &str) {
        // update index in memory
        let result = hash_map_index.delete(key.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let hash_map_index = hash_map_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&sync_bytes).unwrap();
        // read from log file
        let bytes_in_log_file = read_full_file(tmp_file_path);
        let hash_map_index_from_file = UniqueHashMapIndex::from_bytes(&bytes_in_log_file);
        assert!(hash_map_index_from_file.is_ok());
        // verify if log_file is correct
        let hash_map_index_from_file = hash_map_index_from_file.unwrap();
        assert_eq!(
            hash_map_index.index_clone(),
            hash_map_index_from_file.index_clone()
        );
    }

    // let tmp_file_path = "./tmp/unqiue_hash_map_index_sync_new_log_file.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("unqiue_hash_map_index_sync_new_log_file.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();

    // common index
    let mut hash_map_index = UniqueHashMapIndex::from_bytes(&[]).unwrap();

    insert_tuple_test(&mut hash_map_index, tmp_file_path, "1x1", "1");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "2x2", "4");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "3x3", "9");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "4x4", "16");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "5x5", "25");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "6x6", "36");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "7x7", "49");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "8x8", "64");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "9x9", "81");
    insert_tuple_test(&mut hash_map_index, tmp_file_path, "10x10", "100");

    delete_tuple_test(&mut hash_map_index, tmp_file_path, "1x1");
    delete_tuple_test(&mut hash_map_index, tmp_file_path, "2x2");
    delete_tuple_test(&mut hash_map_index, tmp_file_path, "3x3");
    delete_tuple_test(&mut hash_map_index, tmp_file_path, "4x4");
    delete_tuple_test(&mut hash_map_index, tmp_file_path, "5x5");
    delete_tuple_test(&mut hash_map_index, tmp_file_path, "6x6");
    delete_tuple_test(&mut hash_map_index, tmp_file_path, "7x7");
    delete_tuple_test(&mut hash_map_index, tmp_file_path, "8x8");
    delete_tuple_test(&mut hash_map_index, tmp_file_path, "9x9");
    delete_tuple_test(&mut hash_map_index, tmp_file_path, "10x10");

    // clean up
    remove_dir_contents(tmp_dir_path);
}
