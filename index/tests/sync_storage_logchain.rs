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
fn btree_index_sync_storage_logchain() {
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("btree_index_sync_storage_logchain_0.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();

    // create new storage at tmp_file_path
    use storage::Storage;
    let block_len = 16;
    let mut storage = Storage::new(tmp_file_path.to_string(), block_len).unwrap();

    // create new log chain
    let result = logchain::create_log(&mut storage, &(vec![] as Vec<u8>));
    assert!(result.is_ok());
    let (first_block_index, last_block_index) = result.unwrap();
    assert_eq!(first_block_index, last_block_index);

    // read storage file
    let data = read_full_file(tmp_file_path);
    // [16, 0, 0, 0, 4, 0, 0, 0, 255, 255, 255, 255]
    assert_eq!(
        data,
        vec![
            // block_len
            16, 0, 0, 0, // First block
            // - block_data_size
            4, 0, 0, 0, // - block_data = next_block_index in logchain
            255, 255, 255, 255
        ]
    );

    // btree index
    let mut btree_index = BTreeIndex::from_bytes(&[]).unwrap();

    fn insert_tuple_test(
        btree_index: &mut BTreeIndex,
        storage: &mut storage::Storage,
        log_first_block_index: storage::BlockIndex,
        key: &str,
        value: &str,
    ) -> (storage::BlockIndex, storage::BlockIndex) {
        // add to index in memory
        let result = btree_index.insert(key.as_bytes().to_vec(), value.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let btree_index = btree_index.clone();
        // append logchain
        logchain::append_log(storage, log_first_block_index, &sync_bytes).unwrap();
        // read log
        let result = logchain::read_log(storage, log_first_block_index);
        let (first_block_index, last_block_index, bytes_in_log_file) = result.unwrap();
        let btree_index_from_file = BTreeIndex::from_bytes(&bytes_in_log_file);
        assert!(btree_index_from_file.is_ok());
        // verify if log_file is correct
        let btree_index_from_file = btree_index_from_file.unwrap();
        assert_eq!(
            btree_index.index_clone(),
            btree_index_from_file.index_clone()
        );
        (first_block_index, last_block_index)
    }

    fn remove_tuple_test(
        btree_index: &mut BTreeIndex,
        storage: &mut storage::Storage,
        log_first_block_index: storage::BlockIndex,
        key: &str,
        value: &str,
    ) -> (storage::BlockIndex, storage::BlockIndex) {
        // remove from index in memory
        let result = btree_index.remove(key.as_bytes().to_vec(), value.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let btree_index = btree_index.clone();
        // append logchain
        logchain::append_log(storage, log_first_block_index, &sync_bytes).unwrap();
        // read log
        let result = logchain::read_log(storage, log_first_block_index);
        let (first_block_index, last_block_index, bytes_in_log_file) = result.unwrap();
        let btree_index_from_file = BTreeIndex::from_bytes(&bytes_in_log_file);
        assert!(btree_index_from_file.is_ok());
        // verify if log_file is correct
        let btree_index_from_file = btree_index_from_file.unwrap();
        assert_eq!(
            btree_index.index_clone(),
            btree_index_from_file.index_clone()
        );
        (first_block_index, last_block_index)
    }

    fn delete_tuple_with_key_test(
        btree_index: &mut BTreeIndex,
        storage: &mut storage::Storage,
        log_first_block_index: storage::BlockIndex,
        key: &str,
    ) -> (storage::BlockIndex, storage::BlockIndex) {
        // delete from index in memory
        let result = btree_index.delete(key.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let btree_index = btree_index.clone();
        // append logchain
        logchain::append_log(storage, log_first_block_index, &sync_bytes).unwrap();
        // read log
        let result = logchain::read_log(storage, log_first_block_index);
        let (first_block_index, last_block_index, bytes_in_log_file) = result.unwrap();
        let btree_index_from_file = BTreeIndex::from_bytes(&bytes_in_log_file);
        assert!(btree_index_from_file.is_ok());
        // verify if log_file is correct
        let btree_index_from_file = btree_index_from_file.unwrap();
        assert_eq!(
            btree_index.index_clone(),
            btree_index_from_file.index_clone()
        );
        (first_block_index, last_block_index)
    }

    // insert some tuples
    [
        ("1x1", "1"),
        ("1x1", "One"),
        ("2x2", "4"),
        ("2x2", "Four"),
        ("3x3", "9"),
        ("3x3", "Nine"),
        ("4x4", "16"),
        ("4x4", "Sixteen"),
        ("5x5", "25"),
        ("5x5", "Twenty-five"),
        ("6x6", "36"),
        ("6x6", "Thirty-six"),
        ("7x7", "49"),
        ("7x7", "Forty-nine"),
        ("8x8", "64"),
        ("8x8", "Sixty-four"),
        ("9x9", "81"),
        ("9x9", "Eighty-one"),
        ("10x10", "100"),
        ("10x10", "One-hundred"),
    ]
    .iter()
    .for_each(|(key, value)| {
        let result = insert_tuple_test(
            &mut btree_index,
            &mut storage,
            first_block_index,
            key,
            value,
        );
        assert_eq!(result.0, first_block_index);
        assert!(result.1 >= first_block_index);
    });

    // remove some tuples
    [
        ("1x1", "One"),
        ("2x2", "Four"),
        ("3x3", "Nine"),
        ("4x4", "Sixteen"),
        ("5x5", "Twenty-five"),
        ("6x6", "Thirty-six"),
        ("7x7", "Forty-nine"),
        ("8x8", "Sixty-four"),
        ("9x9", "Eighty-one"),
        ("10x10", "One-hundred"),
    ]
    .iter()
    .for_each(|(key, value)| {
        let result = remove_tuple_test(
            &mut btree_index,
            &mut storage,
            first_block_index,
            key,
            value,
        );
        assert_eq!(result.0, first_block_index);
        assert!(result.1 >= first_block_index);
    });

    // remove all remaining tupples of key "1x1", "2x2", "3x3"
    [("1x1", "1"), ("2x2", "4"), ("3x3", "9")]
        .iter()
        .for_each(|(key, value)| {
            let result = remove_tuple_test(
                &mut btree_index,
                &mut storage,
                first_block_index,
                key,
                value,
            );
            assert_eq!(result.0, first_block_index);
            assert!(result.1 >= first_block_index);
        });

    // delete remaining all keys
    ["4x4", "5x5", "6x6", "7x7", "8x8", "9x9", "10x10"]
        .iter()
        .for_each(|key| {
            let result =
                delete_tuple_with_key_test(&mut btree_index, &mut storage, first_block_index, key);
            assert_eq!(result.0, first_block_index);
            assert!(result.1 >= first_block_index);
        });

    // check if btree is empty
    assert_eq!(btree_index.index_clone().len(), 0);

    // clean up
    remove_dir_contents(tmp_dir_path);
}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

// ... ... ... ... ... ... UniqueBTreeIndex ... ... ... ... ... .

#[test]
fn unqiue_btree_index_sync_storage_logchain() {
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("unqiue_btree_index_sync_storage_logchain_0.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();

    // create new storage at tmp_file_path
    use storage::Storage;
    let block_len = 16;
    let mut storage = Storage::new(tmp_file_path.to_string(), block_len).unwrap();

    // create new log chain
    let result = logchain::create_log(&mut storage, &(vec![] as Vec<u8>));
    assert!(result.is_ok());
    let (first_block_index, last_block_index) = result.unwrap();
    assert_eq!(first_block_index, last_block_index);

    // read storage file
    let data = read_full_file(tmp_file_path);
    // [16, 0, 0, 0, 4, 0, 0, 0, 255, 255, 255, 255]
    assert_eq!(
        data,
        vec![
            // block_len
            16, 0, 0, 0, // First block
            // - block_data_size
            4, 0, 0, 0, // - block_data = next_block_index in logchain
            255, 255, 255, 255
        ]
    );

    // unique btree index
    let mut btree_index = UniqueBTreeIndex::from_bytes(&[]).unwrap();

    fn insert_tuple_test(
        btree_index: &mut UniqueBTreeIndex,
        storage: &mut storage::Storage,
        log_first_block_index: storage::BlockIndex,
        key: &str,
        value: &str,
    ) -> (storage::BlockIndex, storage::BlockIndex) {
        // add to index in memory
        let result = btree_index.set(key.as_bytes().to_vec(), value.as_bytes().to_vec(), false);
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let btree_index = btree_index.clone();
        // append logchain
        logchain::append_log(storage, log_first_block_index, &sync_bytes).unwrap();
        // read log
        let result = logchain::read_log(storage, log_first_block_index);
        let (first_block_index, last_block_index, bytes_in_log_file) = result.unwrap();
        let btree_index_from_file = UniqueBTreeIndex::from_bytes(&bytes_in_log_file);
        assert!(btree_index_from_file.is_ok());
        // verify if log_file is correct
        let btree_index_from_file = btree_index_from_file.unwrap();
        assert_eq!(
            btree_index.index_clone(),
            btree_index_from_file.index_clone()
        );
        (first_block_index, last_block_index)
    }

    fn delete_tuple_with_key_test(
        btree_index: &mut UniqueBTreeIndex,
        storage: &mut storage::Storage,
        log_first_block_index: storage::BlockIndex,
        key: &str,
    ) -> (storage::BlockIndex, storage::BlockIndex) {
        // delete from index in memory
        let result = btree_index.delete(key.as_bytes().to_vec());
        assert!(result.is_ok());
        let sync_bytes = result.unwrap();
        let btree_index = btree_index.clone();
        // append logchain
        logchain::append_log(storage, log_first_block_index, &sync_bytes).unwrap();
        // read log
        let result = logchain::read_log(storage, log_first_block_index);
        let (first_block_index, last_block_index, bytes_in_log_file) = result.unwrap();
        let btree_index_from_file = UniqueBTreeIndex::from_bytes(&bytes_in_log_file);
        assert!(btree_index_from_file.is_ok());
        // verify if log_file is correct
        let btree_index_from_file = btree_index_from_file.unwrap();
        assert_eq!(
            btree_index.index_clone(),
            btree_index_from_file.index_clone()
        );
        (first_block_index, last_block_index)
    }

    let tupples = [
        ("1x1", "One"),
        ("2x2", "Four"),
        ("3x3", "Nine"),
        ("4x4", "Sixteen"),
        ("5x5", "Twenty-five"),
        ("6x6", "Thirty-six"),
        ("7x7", "Forty-nine"),
        ("8x8", "Sixty-four"),
        ("9x9", "Eighty-one"),
        ("10x10", "One-hundred"),
    ];

    // insert some tuples
    tupples.iter().for_each(|tuple| {
        let result = insert_tuple_test(
            &mut btree_index,
            &mut storage,
            first_block_index,
            tuple.0,
            tuple.1,
        );
        assert_eq!(result.0, first_block_index);
        assert!(result.1 >= first_block_index);
    });

    // delete all keys
    tupples.iter().for_each(|tuple| {
        let result =
            delete_tuple_with_key_test(&mut btree_index, &mut storage, first_block_index, tuple.0);
        assert_eq!(result.0, first_block_index);
        assert!(result.1 >= first_block_index);
    });

    // check if btree is empty
    assert_eq!(btree_index.index_clone().len(), 0);

    // clean up
    remove_dir_contents(tmp_dir_path);
}
