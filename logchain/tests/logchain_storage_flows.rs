use logchain::{append_log, create_log, delete_log, make_segment_payload_list};
use storage::{BlockIndex, Storage};

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

const SIZE_OF_BLOCK_INDEX: usize = std::mem::size_of::<BlockIndex>();

#[test]
fn make_segment_payload_list_new_storage() {
    fn fetch_state(state_file: &str) -> Vec<u8> {
        use std::path::PathBuf;
        let path: PathBuf = [
            "tests/samples/make_segment_payload_list_new_storage",
            state_file,
        ]
        .iter()
        .collect();
        read_full_file(path.to_str().unwrap())
    }
    // let tmp_file_path = "./tmp/make_segment_payload_list_new_storage.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("make_segment_payload_list_new_storage.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();
    // create new storage
    let block_len = 12;
    let storage_result = Storage::new(String::from(tmp_file_path), block_len);
    assert_eq!(storage_result.is_ok(), true);
    let mut storage = storage_result.unwrap();
    let expected = fetch_state("on_create.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(actual, expected);
    // check log 0
    let log_0_data = vec![
        1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8, 8 as u8, 9 as u8, 10 as u8,
        11 as u8, 12 as u8, 13 as u8, 14 as u8, 15 as u8, 16 as u8,
    ];
    let result = make_segment_payload_list(&mut storage, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let (segment_list, first_block_index, last_block_index) = result.unwrap();
    // ([(0, [1, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8]), (1, [255, 255, 255, 255, 9, 10, 11, 12, 13, 14, 15, 16])], 0, 1)
    assert_eq!(first_block_index, 0);
    assert_eq!(last_block_index, 1);
    assert_eq!(segment_list.len(), 2);
    assert_eq!(segment_list[0].0, 0);
    assert_eq!(segment_list[0].1.len(), block_len as usize);
    assert_eq!(segment_list[1].0, 1);
    assert_eq!(segment_list[1].1.len(), block_len as usize);
    // check log 1
    let log_1_data = vec![
        17 as u8, 18 as u8, 19 as u8, 20 as u8, 21 as u8, 22 as u8, 23 as u8, 24 as u8, 25 as u8,
        26 as u8, 27 as u8, 28 as u8, 29 as u8, 30 as u8, 31 as u8, 32 as u8, 33 as u8, 34 as u8,
        35 as u8, 36 as u8, 37 as u8, 38 as u8, 39 as u8, 40 as u8, 41 as u8, 42 as u8, 43 as u8,
        44 as u8, 45 as u8, 46 as u8, 47 as u8, 48 as u8, 49 as u8, 50 as u8, 51 as u8, 52 as u8,
    ];
    let result = make_segment_payload_list(&mut storage, &log_1_data);
    assert_eq!(result.is_ok(), true);
    let (segment_list, first_block_index, last_block_index) = result.unwrap();
    // ([(0, [(0, [1, 0, 0, 0, 17, 18, 19, 20, 21, 22, 23, 24]), (1, [2, 0, 0, 0, 25, 26, 27, 28, 29, 30, 31, 32]), (2, [3, 0, 0, 0, 33, 34, 35, 36, 37, 38, 39, 40]), (3, [4, 0, 0, 0, 41, 42, 43, 44, 45, 46, 47, 48]), (4, [255, 255, 255, 255, 49, 50, 51, 52])], 0, 4)
    assert_eq!(first_block_index, 0);
    assert_eq!(last_block_index, 4);
    assert_eq!(segment_list.len(), 5);
    assert_eq!(segment_list[0].0, 0);
    assert_eq!(segment_list[0].1.len(), block_len as usize);
    assert_eq!(segment_list[1].0, 1);
    assert_eq!(segment_list[1].1.len(), block_len as usize);
    assert_eq!(segment_list[2].0, 2);
    assert_eq!(segment_list[2].1.len(), block_len as usize);
    assert_eq!(segment_list[3].0, 3);
    assert_eq!(segment_list[3].1.len(), block_len as usize);
    assert_eq!(segment_list[4].0, 4);
    assert_eq!(
        segment_list[4].1.len(),
        SIZE_OF_BLOCK_INDEX
            + (log_1_data.len() % (block_len as usize - SIZE_OF_BLOCK_INDEX) as usize) as usize
    );
    remove_dir_contents(std::path::PathBuf::from(tmp_dir_path));
}

#[test]
fn make_segment_payload_list_existing_storage() {
    // let tmp_file_path = "./tmp/make_segment_payload_list_existing_storage.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("make_segment_payload_list_existing_storage.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();
    // copy "tests/samples/make_segment_payload_list_existing_storage/w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2_w-3_w-4_w-5_sd-3.hex" to tmp_file_path
    let mut src_path =
        std::path::PathBuf::from("tests/samples/make_segment_payload_list_existing_storage");
    src_path.push("w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2_w-3_w-4_w-5_sd-3.hex");
    std::fs::copy(src_path, tmp_file_path.clone()).unwrap();
    let mut storage = Storage::open(String::from(tmp_file_path)).unwrap();
    let block_len = storage.block_len();
    // check log 0
    let log_0_data = vec![
        1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8, 8 as u8, 9 as u8, 10 as u8,
        11 as u8, 12 as u8, 13 as u8, 14 as u8, 15 as u8, 16 as u8,
    ];
    let result = make_segment_payload_list(&mut storage, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let (segment_list, first_block_index, last_block_index) = result.unwrap();
    // ([(0, [1, 0, 0, 0, 1, 2, 3, 4]), (1, [3, 0, 0, 0, 5, 6, 7, 8]), (3, [6, 0, 0, 0, 9, 10, 11, 12]), (6, [255, 255, 255, 255, 13, 14, 15, 16])], 0 , 6)
    assert_eq!(first_block_index, 0);
    assert_eq!(last_block_index, 6);
    assert_eq!(segment_list.len(), 4);
    assert_eq!(segment_list[0].0, 0);
    assert_eq!(segment_list[0].1.len(), block_len as usize);
    assert_eq!(segment_list[1].0, 1);
    assert_eq!(segment_list[1].1.len(), block_len as usize);
    assert_eq!(segment_list[2].0, 3);
    assert_eq!(segment_list[2].1.len(), block_len as usize);
    assert_eq!(segment_list[3].0, 6);
    assert_eq!(segment_list[3].1.len(), block_len as usize,);
    // check log 1
    let log_1_data = vec![
        17 as u8, 18 as u8, 19 as u8, 20 as u8, 21 as u8, 22 as u8, 23 as u8, 24 as u8, 25 as u8,
        26 as u8, 27 as u8, 28 as u8, 29 as u8, 30 as u8, 31 as u8, 32 as u8, 33 as u8, 34 as u8,
        35 as u8, 36 as u8, 37 as u8, 38 as u8, 39 as u8, 40 as u8, 41 as u8, 42 as u8, 43 as u8,
        44 as u8, 45 as u8, 46 as u8, 47 as u8, 48 as u8, 49 as u8, 50 as u8, 51 as u8, 52 as u8,
    ];
    let result = make_segment_payload_list(&mut storage, &log_1_data);
    assert_eq!(result.is_ok(), true);
    let (segment_list, first_block_index, last_block_index) = result.unwrap();
    // ([(0, [1, 0, 0, 0, 17, 18, 19, 20]), (1, [3, 0, 0, 0, 21, 22, 23, 24]), (3, [6, 0, 0, 0, 25, 26, 27, 28]), (6, [7, 0, 0, 0, 29, 30, 31, 32]), (7, [8, 0, 0, 0, 33, 34, 35, 36]), (8, [9, 0, 0, 0, 37, 38, 39, 40]), (9, [10, 0, 0, 0, 41, 42, 43, 44]), (10, [11, 0, 0, 0, 45, 46, 47, 48]), (11, [255, 255, 255, 255, 49, 50, 51, 52])], 0, 11)
    assert_eq!(first_block_index, 0);
    assert_eq!(last_block_index, 11);
    assert_eq!(segment_list.len(), 9);
    assert_eq!(segment_list[0].0, 0);
    assert_eq!(segment_list[0].1.len(), block_len as usize);
    assert_eq!(segment_list[1].0, 1);
    assert_eq!(segment_list[1].1.len(), block_len as usize);
    assert_eq!(segment_list[2].0, 3);
    assert_eq!(segment_list[2].1.len(), block_len as usize);
    assert_eq!(segment_list[3].0, 6);
    assert_eq!(segment_list[3].1.len(), block_len as usize);
    assert_eq!(segment_list[4].0, 7);
    assert_eq!(segment_list[4].1.len(), block_len as usize);
    assert_eq!(segment_list[5].0, 8);
    assert_eq!(segment_list[5].1.len(), block_len as usize);
    assert_eq!(segment_list[6].0, 9);
    assert_eq!(segment_list[6].1.len(), block_len as usize);
    assert_eq!(segment_list[7].0, 10);
    assert_eq!(segment_list[7].1.len(), block_len as usize);
    assert_eq!(segment_list[8].0, 11);
    assert_eq!(segment_list[8].1.len(), block_len as usize);
    remove_dir_contents(std::path::PathBuf::from(tmp_dir_path));
}

#[test]
fn create_log_new_storage() {
    fn fetch_state(state_file: &str) -> Vec<u8> {
        use std::path::PathBuf;
        let path: PathBuf = ["tests/samples/create_log_new_storage", state_file]
            .iter()
            .collect();
        read_full_file(path.to_str().unwrap())
    }
    // let tmp_file_path = "./tmp/create_log_new_storage.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("create_log_new_storage.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();

    // create new storage
    let block_len = 8;
    let storage_result = Storage::new(String::from(tmp_file_path), block_len);
    assert_eq!(storage_result.is_ok(), true);
    let mut storage = storage_result.unwrap();
    let expected = fetch_state("on_create.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(actual, expected);

    // write log 0
    let log_0_data = vec![
        1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8, 8 as u8, 9 as u8, 10 as u8,
        11 as u8, 12 as u8, 13 as u8, 14 as u8, 15 as u8, 16 as u8,
    ];
    let result = create_log(&mut storage, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    // (0, 3)
    assert_eq!(first_block_index, 0);
    assert_eq!(last_block_index, 3);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state("add-log-0.hex");
    assert_eq!(actual, expected);
    // write log 1
    let log_1_data = vec![
        17 as u8, 18 as u8, 19 as u8, 20 as u8, 21 as u8, 22 as u8, 23 as u8, 24 as u8, 25 as u8,
        26 as u8, 27 as u8, 28 as u8, 29 as u8, 30 as u8, 31 as u8, 32 as u8, 33 as u8, 34 as u8,
        35 as u8, 36 as u8, 37 as u8, 38 as u8, 39 as u8, 40 as u8, 41 as u8, 42 as u8, 43 as u8,
        44 as u8, 45 as u8, 46 as u8, 47 as u8, 48 as u8, 49 as u8, 50 as u8, 51 as u8, 52 as u8,
    ];
    let result = create_log(&mut storage, &log_1_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    // (4, 12)
    assert_eq!(first_block_index, 4);
    assert_eq!(last_block_index, 12);
    let expected = fetch_state("add-log-0_add-log-1.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(actual, expected);
    remove_dir_contents(std::path::PathBuf::from(tmp_dir_path));
}

#[test]
fn create_log_existing_storage() {
    fn fetch_state(state_file: &str) -> Vec<u8> {
        use std::path::PathBuf;
        let path: PathBuf = ["tests/samples/create_log_existing_storage", state_file]
            .iter()
            .collect();
        read_full_file(path.to_str().unwrap())
    }
    // let tmp_file_path = "./tmp/create_log_existing_storage.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("create_log_existing_storage.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();
    // copy "tests/samples/make_segment_payload_list_existing_storage/w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2_w-3_w-4_w-5_sd-3.hex" to tmp_file_path
    let mut src_path = std::path::PathBuf::from("tests/samples/create_log_existing_storage");
    src_path.push("w-0_w-1_w-2_sd-0_hd-0_sd-1_hd-2_w-2_w-3_w-4_w-5_sd-3.hex");
    std::fs::copy(src_path, tmp_file_path.clone()).unwrap();
    let mut storage = Storage::open(String::from(tmp_file_path)).unwrap();
    let block_len = storage.block_len();
    let actual = read_full_file(tmp_file_path);

    // write log 0
    let log_0_data = vec![
        1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8, 8 as u8, 9 as u8, 10 as u8,
        11 as u8, 12 as u8, 13 as u8, 14 as u8, 15 as u8, 16 as u8,
    ];
    let result = create_log(&mut storage, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    // (0, 6)
    assert_eq!(first_block_index, 0);
    assert_eq!(last_block_index, 6);
    let expected = fetch_state("--add-log-0.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(actual, expected);
    // write log 1
    let log_1_data = vec![
        17 as u8, 18 as u8, 19 as u8, 20 as u8, 21 as u8, 22 as u8, 23 as u8, 24 as u8, 25 as u8,
        26 as u8, 27 as u8, 28 as u8, 29 as u8, 30 as u8, 31 as u8, 32 as u8, 33 as u8, 34 as u8,
        35 as u8, 36 as u8, 37 as u8, 38 as u8, 39 as u8, 40 as u8, 41 as u8, 42 as u8, 43 as u8,
        44 as u8, 45 as u8, 46 as u8, 47 as u8, 48 as u8, 49 as u8, 50 as u8, 51 as u8, 52 as u8,
    ];
    let result = create_log(&mut storage, &log_1_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    // (7, 15)
    assert_eq!(first_block_index, 7);
    assert_eq!(last_block_index, 15);
    let expected = fetch_state("--add-log-0_add-log-1.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(actual, expected);
    remove_dir_contents(std::path::PathBuf::from(tmp_dir_path));
}

fn append_log_new_storage() {}

fn append_log_existing_storage() {}

fn delete_log_new_storage() {}

fn delete_log_existing_storage() {}
