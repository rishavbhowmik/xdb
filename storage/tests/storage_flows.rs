use storage::Storage;

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
fn storage_open_new_file() {
    fn fetch_state(state_file: &str) -> Vec<u8> {
        use std::path::PathBuf;
        let path: PathBuf = ["tests/samples/storage_open_new_file_states", state_file]
            .iter()
            .collect();
        read_full_file(path.to_str().unwrap())
    }
    // let tmp_file_path = "./tmp/storage_open_new_file.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("storage_open_new_file.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();
    // create new storage
    let storage_result = Storage::new(String::from(tmp_file_path), 8);
    assert_eq!(storage_result.is_ok(), true);
    let mut storage = storage_result.unwrap();
    let expected = fetch_state("on_create.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // write to block 0
    let block_0_data = vec![
        1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8, 8 as u8,
    ];
    let result = storage.write_block(0, &block_0_data);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 16); // 4 + (4 + 8) * 0 + 4 + 8
    let expected = fetch_state("on_write_block_0.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // write to block 1
    let block_1_data = vec![
        9 as u8, 10 as u8, 11 as u8, 12 as u8, 13 as u8, 14 as u8, 15 as u8, 16 as u8,
    ];
    let result = storage.write_block(1, &block_1_data);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 28); // 4 + (4 + 8) * 1 + 4 + 8
    let expected = fetch_state("on_write_block_1.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // write to block 2
    let block_2_data = vec![17 as u8, 18 as u8, 19 as u8, 20 as u8];
    let result = storage.write_block(2, &block_2_data);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 36); // 4 + (4 + 8) * 2 + 4 + 4
    let expected = fetch_state("on_write_block_2.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // read from block 2
    let result = storage.read_block(2);
    assert_eq!(result.is_ok(), true);
    let (read_ptr, actual_data) = result.unwrap();
    assert_eq!(read_ptr, 36); // 4 + (4 + 8) * 2 + 4 + 4
    assert_eq!(actual_data, block_2_data);
    // read from block 1
    let result = storage.read_block(1);
    assert_eq!(result.is_ok(), true);
    let (read_ptr, actual_data) = result.unwrap();
    assert_eq!(read_ptr, 28); // 4 + (4 + 8) * 1 + 4 + 8
    assert_eq!(actual_data, block_1_data);
    // read from block 0
    let result = storage.read_block(0);
    assert_eq!(result.is_ok(), true);
    let (read_ptr, actual_data) = result.unwrap();
    assert_eq!(read_ptr, 16); // 4 + (4 + 8) * 0 + 4 + 8
    assert_eq!(actual_data, block_0_data);
    // read from block 3
    let result = storage.read_block(3);
    assert_eq!(result.is_ok(), true);
    let (read_ptr, actual_data) = result.unwrap();
    assert_eq!(read_ptr, 16); // no change
    assert_eq!(actual_data.len(), 0); // no data
                                      // soft delete_block 0
    let result = storage.delete_block(0, false);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 8); // 4 + (4 + 8) * 0 + 4 + 0
    let expected = fetch_state("on_soft_delete_block_0.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // hard delete_block 0
    let result = storage.delete_block(0, true);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 16); // 4 + (4 + 8) * 0 + 4 + 8
    let expected = fetch_state("on_hard_delete_block_0.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // soft delete_block 1
    let result = storage.delete_block(1, false);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 20); // 4 + (4 + 8) * 1 + 4 + 0
    let expected = fetch_state("on_soft_delete_block_1.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // hard delete_block 2
    let result = storage.delete_block(2, true);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 40); // 4 + (4 + 8) * 2 + 4 + 8
    let expected = fetch_state("on_hard_delete_block_2.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);

    // clear clutter
    remove_dir_contents(std::path::PathBuf::from(tmp_dir_path));
}

#[test]
fn storage_open_existing_file1() {
    fn fetch_state(state_file: &str) -> Vec<u8> {
        use std::path::PathBuf;
        let path: PathBuf = ["tests/samples/storage_open_existing_file1", state_file]
            .iter()
            .collect();
        read_full_file(path.to_str().unwrap())
    }
    // let tmp_file_path = "./tmp/storage_open_existing_file1.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("storage_open_existing_file1.hex"),
    ]
    .iter()
    .collect();
    // copy "tests/samples/storage_open_existing_file1/w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2.hex" to tmp_file_path
    let mut src_path = std::path::PathBuf::from("tests/samples/storage_open_existing_file1");
    src_path.push("w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2.hex");
    std::fs::copy(src_path, tmp_file_path.clone()).unwrap();
    let tmp_file_path = tmp_file_path.to_str().unwrap();
    // open storage
    let mut storage = Storage::open(String::from(tmp_file_path)).unwrap();
    // read from block 0
    let result = storage.read_block(0);
    assert_eq!(result.is_ok(), true);
    let (_, actual_data) = result.unwrap();
    assert_eq!(actual_data.len(), 0); // no data
                                      // read from block 1
    let result = storage.read_block(1);
    assert_eq!(result.is_ok(), true);
    let (_, actual_data) = result.unwrap();
    assert_eq!(actual_data.len(), 0); // no data
                                      // read from block 2
    let result = storage.read_block(2);
    assert_eq!(result.is_ok(), true);
    let (read_ptr, actual_data) = result.unwrap();
    assert_eq!(read_ptr, 36); // 4 + (4 + 8) * 2 + 4 + 4
    let block_2_data = vec![17 as u8, 18 as u8, 19 as u8, 20 as u8];
    assert_eq!(actual_data, block_2_data); // no data
                                           // read from block 3
    let result = storage.read_block(3);
    assert_eq!(result.is_ok(), true);
    let (read_ptr, actual_data) = result.unwrap();
    assert_eq!(read_ptr, 36); // no change
    assert_eq!(actual_data.len(), 0); // no data

    // write to block 3
    let block_3_data = vec![3 as u8, 9 as u8, 27 as u8];
    let result = storage.write_block(3, &block_3_data);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 47); // 4 + (4 + 8) * 3 + 4 + 3
    let expected = fetch_state("w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2_w-3.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // write to block 4
    let block_4_data = vec![4 as u8, 8 as u8, 16 as u8, 32 as u8];
    let result = storage.write_block(4, &block_4_data);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 60); // 4 + (4 + 8) * 4 + 4 + 4
    let expected = fetch_state("w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2_w-3_w-4.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // write to block 5
    let block_5_data = vec![5 as u8, 10 as u8, 20 as u8, 40 as u8, 80 as u8];
    let result = storage.write_block(5, &block_5_data);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 73); // 4 + (4 + 8) * 5 + 4 + 5
    let expected = fetch_state("w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2_w-3_w-4_w-5.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // TODO:
    // soft delete block 1
    let result = storage.delete_block(1, false);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 73); // no change
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // soft delete block 3
    let result = storage.delete_block(3, false);
    assert_eq!(result.is_ok(), true);
    let write_ptr = result.unwrap();
    assert_eq!(write_ptr, 44); // 4 + (4 + 8) * 3 + 4
    let expected = fetch_state("w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2_w-3_w-4_w-5_sd-3.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(expected, actual);
    // clear clutter
    remove_dir_contents(std::path::PathBuf::from(tmp_dir_path));
}

#[test]
fn storage_open_existing_file2() {}

#[test]
fn storage_open_existing_file1_test_abstract_fn() {
    // let tmp_file_path = "./tmp/storage_open_existing_file1.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("storage_open_existing_file1.hex"),
    ]
    .iter()
    .collect();
    // copy "tests/samples/storage_open_existing_file1/w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2.hex" to tmp_file_path
    let mut src_path = std::path::PathBuf::from("tests/samples/storage_open_existing_file1");
    src_path.push("w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2.hex");
    std::fs::copy(src_path, tmp_file_path.clone()).unwrap();
    let tmp_file_path = tmp_file_path.to_str().unwrap();
    // open storage
    let storage = Storage::open(String::from(tmp_file_path)).unwrap();
    // available free blocks: {0, 1}, endblock: 2
    // - search for 1 block
    let expected = vec![0 as u32];
    let actual = storage.search_block_allocation_indexes(1);
    assert_eq!(expected, actual);
    // - search for 2 blocks
    let expected = vec![0, 1];
    let actual = storage.search_block_allocation_indexes(2);
    assert_eq!(expected, actual);
    // - search for 3 blocks
    let expected = vec![0, 1, 3];
    let actual = storage.search_block_allocation_indexes(3);
    assert_eq!(expected, actual);
    // - search for 4 blocks
    let expected = vec![0, 1, 3, 4];
    let actual = storage.search_block_allocation_indexes(4);
    assert_eq!(expected, actual);
    // - search for 5 blocks
    let expected = vec![0, 1, 3, 4, 5];
    let actual = storage.search_block_allocation_indexes(5);
    assert_eq!(expected, actual);
}
