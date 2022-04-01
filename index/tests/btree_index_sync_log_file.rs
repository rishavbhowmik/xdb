use index::index_traits::{IndexCloneTrait, IndexSerializationTrait, IndexTrait, UniqueIndexTrait};
use index::kv::tuple::KVTuple;
use index::{BTreeIndex, UniqueBTreeIndex};

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

#[test]
fn btree_index_sync_new_log_file() {
    /// Test that the log file is synced after each operation.
    /// - Add a key-value pair to the ref-index in memory.
    /// - Append the key-value pair buffer to the log file.
    /// - Read the log file and retrive index from it.
    /// - Verify that the retrive index from log file is same as the ref-index in memory.
    fn insert_tuple_test(
        btree_index: &mut BTreeIndex,
        tmp_file_path: &str,
        key: &str,
        value: &str,
    ) {
        let kv_tuple = KVTuple::new_insert(key.as_bytes(), value.as_bytes());
        // add to index in memory
        let result = btree_index.insert(kv_tuple.key().unwrap(), kv_tuple.value().unwrap());
        assert!(result.is_ok());
        let btree_index = btree_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        let bytes = kv_tuple.to_bytes();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&bytes).unwrap();
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

    // let tmp_file_path = "./tmp/btree_index_sync_log_file.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("btree_index_sync_log_file.hex"),
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

    remove_dir_contents(tmp_dir_path);
}

fn btree_index_sync_existing_log_file() {}

#[test]
fn unqiue_btree_index_sync_new_log_file() {
    /// Test that the log file is synced after each operation.
    /// - Add a key-value pair to the ref-index in memory.
    /// - Append the key-value pair buffer to the log file.
    /// - Read the log file and retrive index from it.
    /// - Verify that the retrive index from log file is same as the ref-index in memory.
    fn insert_tuple_test(
        btree_index: &mut UniqueBTreeIndex,
        tmp_file_path: &str,
        key: &str,
        value: &str,
    ) {
        let kv_tuple = KVTuple::new_insert(key.as_bytes(), value.as_bytes());
        // add to index in memory
        let result = btree_index.set(kv_tuple.key().unwrap(), kv_tuple.value().unwrap(), false);
        assert!(result.is_ok());
        let btree_index = btree_index.clone();
        // add to log file
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        let mut file_writer: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(tmp_file_path)
            .unwrap();
        let bytes = kv_tuple.to_bytes();
        file_writer.seek(std::io::SeekFrom::End(0)).unwrap();
        file_writer.write(&bytes).unwrap();
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

    // let tmp
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("btree_index_sync_log_file.hex"),
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
}
