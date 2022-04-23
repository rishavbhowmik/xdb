use logchain::{append_log, create_log, delete_log, make_segment_payload_list, read_log};
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
    remove_dir(path).unwrap();
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
        1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8, 14_u8,
        15_u8, 16_u8,
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
        17_u8, 18_u8, 19_u8, 20_u8, 21_u8, 22_u8, 23_u8, 24_u8, 25_u8, 26_u8, 27_u8, 28_u8, 29_u8,
        30_u8, 31_u8, 32_u8, 33_u8, 34_u8, 35_u8, 36_u8, 37_u8, 38_u8, 39_u8, 40_u8, 41_u8, 42_u8,
        43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8, 49_u8, 50_u8, 51_u8, 52_u8,
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

    // check empty log
    let empty_log_data = vec![];
    let result = make_segment_payload_list(&mut storage, &empty_log_data);
    assert_eq!(result.is_ok(), true);
    let (segment_list, first_block_index, last_block_index) = result.unwrap();
    // [(0, [255, 255, 255, 255])], 0, 0
    assert_eq!(first_block_index, 0);
    assert_eq!(last_block_index, 0);
    assert_eq!(segment_list.len(), 1);
    assert_eq!(segment_list[0].0, 0);
    assert_eq!(segment_list[0].1, vec![255, 255, 255, 255]);

    // clean up
    remove_dir_contents(tmp_dir_path);
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
        1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8, 14_u8,
        15_u8, 16_u8,
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
        17_u8, 18_u8, 19_u8, 20_u8, 21_u8, 22_u8, 23_u8, 24_u8, 25_u8, 26_u8, 27_u8, 28_u8, 29_u8,
        30_u8, 31_u8, 32_u8, 33_u8, 34_u8, 35_u8, 36_u8, 37_u8, 38_u8, 39_u8, 40_u8, 41_u8, 42_u8,
        43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8, 49_u8, 50_u8, 51_u8, 52_u8,
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

    // empty log
    let log_2_data = vec![];
    let result = make_segment_payload_list(&mut storage, &log_2_data);
    assert_eq!(result.is_ok(), true);
    let (segment_list, first_block_index, last_block_index) = result.unwrap();
    // [(0, [255, 255, 255, 255])], 0, 0
    assert_eq!(first_block_index, 0);
    assert_eq!(last_block_index, 0);
    assert_eq!(segment_list.len(), 1);
    assert_eq!(segment_list[0].0, 0);
    assert_eq!(segment_list[0].1, vec![255, 255, 255, 255]);

    // clean up
    remove_dir_contents(tmp_dir_path);
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
        1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8, 14_u8,
        15_u8, 16_u8,
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
        17_u8, 18_u8, 19_u8, 20_u8, 21_u8, 22_u8, 23_u8, 24_u8, 25_u8, 26_u8, 27_u8, 28_u8, 29_u8,
        30_u8, 31_u8, 32_u8, 33_u8, 34_u8, 35_u8, 36_u8, 37_u8, 38_u8, 39_u8, 40_u8, 41_u8, 42_u8,
        43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8, 49_u8, 50_u8, 51_u8, 52_u8,
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

    // write empty log
    let log_2_data = vec![];
    let result = create_log(&mut storage, &log_2_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    // (13, 13)
    assert_eq!(first_block_index, 13);
    assert_eq!(last_block_index, 13);

    // clean up
    remove_dir_contents(tmp_dir_path);
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
    assert_eq!(block_len, 8);

    // write log 0
    let log_0_data = vec![
        1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8, 14_u8,
        15_u8, 16_u8,
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
        17_u8, 18_u8, 19_u8, 20_u8, 21_u8, 22_u8, 23_u8, 24_u8, 25_u8, 26_u8, 27_u8, 28_u8, 29_u8,
        30_u8, 31_u8, 32_u8, 33_u8, 34_u8, 35_u8, 36_u8, 37_u8, 38_u8, 39_u8, 40_u8, 41_u8, 42_u8,
        43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8, 49_u8, 50_u8, 51_u8, 52_u8,
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

    // write empty log
    let log_2_data = vec![];
    let result = create_log(&mut storage, &log_2_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    // (16, 16)
    assert_eq!(first_block_index, 16);
    assert_eq!(last_block_index, 16);

    // clean up
    remove_dir_contents(tmp_dir_path);
}

#[test]
fn append_log_new_storage() {
    fn fetch_state(state_file: &str) -> Vec<u8> {
        use std::path::PathBuf;
        let path: PathBuf = ["tests/samples/append_log_new_storage", state_file]
            .iter()
            .collect();
        read_full_file(path.to_str().unwrap())
    }
    // let tmp_file_path = "./tmp/append_log_new_storage.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("append_log_new_storage.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();

    // create new storage
    let block_len = 12;
    let storage_result = Storage::new(String::from(tmp_file_path), block_len);
    assert_eq!(storage_result.is_ok(), true);
    let mut storage = storage_result.unwrap();
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state("create.hex");
    assert_eq!(actual, expected);

    // create log 0 (starts at block 0)
    let log_0_data = vec![
        1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8, 14_u8,
        15_u8, 16_u8,
    ];
    let result = create_log(&mut storage, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    // (0, 1)
    assert_eq!(first_block_index, 0);
    assert_eq!(last_block_index, 1);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state("create_add-log-0.hex");
    assert_eq!(actual, expected);

    let log_0_first_block_index = first_block_index;

    // append log 0 - [8, 7, 6, 5, 4, 3, 2, 1] - cover full new block with single traversal
    let log_0_data = vec![8_u8, 7_u8, 6_u8, 5_u8, 4_u8, 3_u8, 2_u8, 1_u8];
    let result = append_log(&mut storage, last_block_index, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 2);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state("create_add-log-0_append-log-0.hex");
    assert_eq!(actual, expected); // ??

    // append log 0 - [8, 7, 6, 5, 4, 3, 2, 1] - cover full new block with traversal from start of log
    let log_0_data = vec![8_u8, 7_u8, 6_u8, 5_u8, 4_u8, 3_u8, 2_u8, 1_u8];
    let result = append_log(&mut storage, 0, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 3);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state("create_add-log-0_append-log-0_append-log-0.hex");
    assert_eq!(actual, expected);

    // append log 0 - [4, 3, 2, 1] - cover partial new block
    let log_0_data = vec![4_u8, 3_u8, 2_u8, 1_u8];
    let result = append_log(&mut storage, last_block_index, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 4);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state("create_add-log-0_append-log-0_append-log-0_append-log-0.hex");
    assert_eq!(actual, expected);

    // append log 0 - [2, 1] - cover partial last block
    let log_0_data = vec![2_u8, 1_u8];
    let result = append_log(&mut storage, last_block_index, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 4);
    let actual = read_full_file(tmp_file_path);
    let expected =
        fetch_state("create_add-log-0_append-log-0_append-log-0_append-log-0_append-log-0.hex");
    assert_eq!(actual, expected);

    // append log 0 - [2,1] - cover partial last block (end of block)
    let log_0_data = vec![2_u8, 1_u8];
    let result = append_log(&mut storage, last_block_index, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 4);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state(
        "create_add-log-0_append-log-0_append-log-0_append-log-0_append-log-0_append-log-0.hex",
    );
    assert_eq!(actual, expected);

    // append log 0 - [4, 3, 2, 1] - cover partial new block
    let log_0_data = vec![4_u8, 3_u8, 2_u8, 1_u8];
    let result = append_log(&mut storage, last_block_index, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 5);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state(
        "create_add-log-0_append-log-0_append-log-0_append-log-0_append-log-0_append-log-0_append-log-0.hex",
    );
    assert_eq!(actual, expected);

    // append log 0 - [8, 7, 6, 5, 4, 3, 2, 1] - cover full new block with no traverse and end at new block
    let log_0_data = vec![8_u8, 7_u8, 6_u8, 5_u8, 4_u8, 3_u8, 2_u8, 1_u8];
    let result = append_log(&mut storage, last_block_index, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 6);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state(
        "create_add-log-0_append-log-0_append-log-0_append-log-0_append-log-0_append-log-0_append-log-0_append-log-0.hex",
    );
    assert_eq!(actual, expected);

    let log_0_last_block_lock = last_block_index;

    // create log 1 - [1, 2, 3, 4, 5, 6, 7, 8]
    let result = create_log(
        &mut storage,
        &[1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8],
    );
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    // (7, 7)
    assert_eq!(first_block_index, 7);
    assert_eq!(last_block_index, 7);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state("create_log-0-trail--create-log-1.hex");
    assert_eq!(actual, expected);

    let log_1_first_block_lock = first_block_index;

    // append log 1 - [32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2]
    let log_1_data = vec![
        32_u8, 31_u8, 30_u8, 29_u8, 28_u8, 27_u8, 26_u8, 25_u8, 24_u8, 23_u8, 22_u8, 21_u8, 20_u8,
        19_u8, 18_u8, 17_u8, 16_u8, 15_u8, 14_u8, 13_u8, 12_u8, 11_u8, 10_u8, 9_u8, 8_u8, 7_u8,
        6_u8, 5_u8, 4_u8, 3_u8, 2_u8,
    ];
    let result = append_log(&mut storage, last_block_index, &log_1_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 11);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state("create_log-0-trail--create-log-1_append-log-1.hex");
    assert_eq!(actual, expected);

    let log_1_last_block_lock = last_block_index;

    // append log 0 - [1]
    let log_0_data = vec![1_u8];
    let result = append_log(&mut storage, log_0_last_block_lock, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, log_0_last_block_lock);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state("create_log-0-trail--create-log-1_append-log-1_append-log-0.hex");
    assert_eq!(actual, expected);

    // append log 0 - [2, 1] - traverse from start
    let log_0_data = vec![2_u8, 1_u8];
    let result = append_log(&mut storage, log_0_first_block_index, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, log_0_last_block_lock);
    let actual = read_full_file(tmp_file_path);
    let expected =
        fetch_state("create_log-0-trail--create-log-1_append-log-1_append-log-0_append-log-0.hex");
    assert_eq!(actual, expected);

    // append log 0 - [8, 7, 6, 5, 4, 3, 2, 1] - traverse from start
    let log_0_data = vec![8_u8, 7_u8, 6_u8, 5_u8, 4_u8, 3_u8, 2_u8, 1_u8];
    let result = append_log(&mut storage, log_0_first_block_index, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 12);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state(
        "create_log-0-trail--create-log-1_append-log-1_append-log-0_append-log-0_append-log-0.hex",
    );
    assert_eq!(actual, expected);

    // append log 1 - [1] - traverse from start
    let log_1_data = vec![5_u8, 4_u8, 3_u8, 2_u8, 1_u8];
    let result = append_log(&mut storage, log_1_first_block_lock, &log_1_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 13);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state(
        "create_log-0-trail--create-log-1_append-log-1_append-log-0_append-log-0_append-log-0_append-log-1.hex",
    );
    assert_eq!(actual, expected);

    // append log 1 - [32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2] - traverse from start
    let log_1_data = vec![
        32_u8, 31_u8, 30_u8, 29_u8, 28_u8, 27_u8, 26_u8, 25_u8, 24_u8, 23_u8, 22_u8, 21_u8, 20_u8,
        19_u8, 18_u8, 17_u8, 16_u8, 15_u8, 14_u8, 13_u8, 12_u8, 11_u8, 10_u8, 9_u8, 8_u8, 7_u8,
        6_u8, 5_u8, 4_u8, 3_u8, 2_u8, 1_u8,
    ];
    let result = append_log(&mut storage, log_1_last_block_lock, &log_1_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, 17);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state(
        "create_log-0-trail--create-log-1_append-log-1_append-log-0_append-log-0_append-log-0_append-log-1_append-log-1.hex",
    );
    assert_eq!(actual, expected);

    let log_1_last_block_lock = last_block_index;

    // append log 1
    let log_1_data = vec![2_u8, 1_u8];
    let result = append_log(&mut storage, log_1_last_block_lock, &log_1_data);
    assert_eq!(result.is_ok(), true);
    let last_block_index = result.unwrap();
    assert_eq!(last_block_index, log_1_last_block_lock);
    let actual = read_full_file(tmp_file_path);
    let expected = fetch_state(
        "create_log-0-trail--create-log-1_append-log-1_append-log-0_append-log-0_append-log-0_append-log-1_append-log-1_append-log-1.hex",
    );
    assert_eq!(actual, expected);

    remove_dir_contents(tmp_dir_path);
}

fn append_log_existing_storage() {}

#[test]
fn delete_log_new_storage() {
    fn fetch_state(state_file: &str) -> Vec<u8> {
        use std::path::PathBuf;
        let path: PathBuf = ["tests/samples/delete_log_new_storage", state_file]
            .iter()
            .collect();
        read_full_file(path.to_str().unwrap())
    }
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("delete_log_new_storage.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();
    let block_len = 8;
    let storage_result = Storage::new(String::from(tmp_file_path), block_len);
    assert_eq!(storage_result.is_ok(), true);
    let mut storage = storage_result.unwrap();

    // write log 0
    let log_0_data = vec![
        1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8, 14_u8,
        15_u8, 16_u8,
    ];
    let result = create_log(&mut storage, &log_0_data);
    assert_eq!(result.is_ok(), true);

    // write log 1
    let log_1_data = vec![
        17_u8, 18_u8, 19_u8, 20_u8, 21_u8, 22_u8, 23_u8, 24_u8, 25_u8, 26_u8, 27_u8, 28_u8, 29_u8,
        30_u8, 31_u8, 32_u8, 33_u8, 34_u8, 35_u8, 36_u8, 37_u8, 38_u8, 39_u8, 40_u8, 41_u8, 42_u8,
        43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8, 49_u8, 50_u8, 51_u8, 52_u8,
    ];
    let result = create_log(&mut storage, &log_1_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    // (4, 12)
    assert_eq!(first_block_index, 4);
    assert_eq!(last_block_index, 12);
    let first_block_index_l1 = first_block_index;
    let last_block_index_l1 = last_block_index;

    // write log 2
    let log_2_data = vec![
        53_u8, 54_u8, 55_u8, 56_u8, 57_u8, 58_u8, 59_u8, 60_u8, 61_u8, 62_u8, 63_u8, 64_u8, 65_u8,
        66_u8, 67_u8, 68_u8, 69_u8, 70_u8, 71_u8, 72_u8, 73_u8, 74_u8, 75_u8, 76_u8, 77_u8, 78_u8,
        79_u8, 80_u8, 81_u8, 82_u8, 83_u8, 84_u8, 85_u8, 86_u8, 87_u8, 88_u8,
    ];
    let result = create_log(&mut storage, &log_2_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();

    // (13, 21)
    assert_eq!(first_block_index, 13);
    assert_eq!(last_block_index, 21);
    let first_block_index_l2 = first_block_index;
    let last_block_index_l2 = last_block_index;

    // write log 3
    let log_3_data = vec![
        89_u8, 90_u8, 91_u8, 92_u8, 93_u8, 94_u8, 95_u8, 96_u8, 97_u8, 98_u8, 99_u8, 100_u8,
        101_u8, 102_u8, 103_u8, 104_u8, 105_u8, 106_u8, 107_u8, 108_u8, 109_u8, 110_u8, 111_u8,
        112_u8, 113_u8, 114_u8, 115_u8, 116_u8, 117_u8, 118_u8, 119_u8, 120_u8, 121_u8, 122_u8,
        123_u8, 124_u8, 125_u8, 126_u8, 127_u8, 128_u8, 129_u8, 130_u8, 131_u8, 132_u8, 133_u8,
        134_u8, 135_u8, 136_u8, 137_u8,
    ];
    let result = create_log(&mut storage, &log_3_data);
    assert_eq!(result.is_ok(), true);
    let expected = fetch_state("4logs.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(actual, expected);

    // delete log 1
    let result = delete_log(&mut storage, first_block_index_l1, true);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    assert_eq!(first_block_index, first_block_index_l1);
    assert_eq!(last_block_index, last_block_index_l1);
    let expected = fetch_state("4logs_del-log1.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(actual, expected);

    // soft delete log 2
    let result = delete_log(&mut storage, first_block_index_l2, false);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    assert_eq!(first_block_index, first_block_index_l2);
    assert_eq!(last_block_index, last_block_index_l2);
    let expected = fetch_state("4logs_del-log1_del-log2.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(actual, expected);
    remove_dir_contents(tmp_dir_path);
}

fn delete_log_existing_storage() {}

#[test]
fn read_log_new_storage() {
    // let tmp_file_path = "./tmp/read_log_new_storage.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from("read_log_new_storage.hex"),
    ]
    .iter()
    .collect();
    let tmp_file_path = tmp_file_path.to_str().unwrap();

    // create new storage
    let block_len = 8;
    let storage_result = Storage::new(String::from(tmp_file_path), block_len);
    assert_eq!(storage_result.is_ok(), true);
    let mut storage = storage_result.unwrap();

    // add logs to storage
    // - log 0
    let log_0_data = vec![
        1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8, 14_u8,
        15_u8, 16_u8,
    ];
    let result = create_log(&mut storage, &log_0_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    let log_0_first_block_index = first_block_index;
    let log_0_last_block_index = last_block_index;
    {
        // test read_block for block_0
        let result = read_log(&mut storage, log_0_first_block_index);
        assert_eq!(result.is_ok(), true);
        let (first_block_index, last_block_index, log_data) = result.unwrap();
        assert_eq!(first_block_index, log_0_first_block_index);
        assert_eq!(last_block_index, log_0_last_block_index);
        assert_eq!(log_data, log_0_data);
    }
    // - log 1
    let log_1_data = vec![
        17_u8, 18_u8, 19_u8, 20_u8, 21_u8, 22_u8, 23_u8, 24_u8, 25_u8, 26_u8, 27_u8, 28_u8, 29_u8,
        30_u8, 31_u8, 32_u8, 33_u8, 34_u8, 35_u8, 36_u8, 37_u8, 38_u8, 39_u8, 40_u8, 41_u8, 42_u8,
        43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 48_u8, 49_u8, 50_u8, 51_u8, 52_u8,
    ];
    let result = create_log(&mut storage, &log_1_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    let log_1_first_block_index = first_block_index;
    let log_1_last_block_index = last_block_index;
    {
        // test read_block for block_1
        let result = read_log(&mut storage, log_1_first_block_index);
        assert_eq!(result.is_ok(), true);
        let (first_block_index, last_block_index, log_data) = result.unwrap();
        assert_eq!(first_block_index, log_1_first_block_index);
        assert_eq!(last_block_index, log_1_last_block_index);
        assert_eq!(log_data, log_1_data);
    }
    // - log 2
    let log_2_data = vec![
        53_u8, 54_u8, 55_u8, 56_u8, 57_u8, 58_u8, 59_u8, 60_u8, 61_u8, 62_u8, 63_u8, 64_u8, 65_u8,
        66_u8, 67_u8, 68_u8, 69_u8, 70_u8, 71_u8, 72_u8, 73_u8, 74_u8, 75_u8, 76_u8, 77_u8, 78_u8,
        79_u8, 80_u8, 81_u8, 82_u8, 83_u8, 84_u8, 85_u8, 86_u8, 87_u8, 88_u8, 89_u8, 90_u8, 91_u8,
        92_u8, 93_u8, 94_u8, 95_u8, 96_u8, 97_u8, 98_u8, 99_u8, 100_u8,
    ];
    let result = create_log(&mut storage, &log_2_data);
    assert_eq!(result.is_ok(), true);
    let (first_block_index, last_block_index) = result.unwrap();
    let log_2_first_block_index = first_block_index;
    let log_2_last_block_index = last_block_index;
    {
        // test read_block for block_2
        let result = read_log(&mut storage, log_2_first_block_index);
        assert_eq!(result.is_ok(), true);
        let (first_block_index, last_block_index, log_data) = result.unwrap();
        assert_eq!(first_block_index, log_2_first_block_index);
        assert_eq!(last_block_index, log_2_last_block_index);
        assert_eq!(log_data, log_2_data);
    }
    {
        // test read_block for block_0
        let result = read_log(&mut storage, log_0_first_block_index);
        assert_eq!(result.is_ok(), true);
        let (first_block_index, last_block_index, log_data) = result.unwrap();
        assert_eq!(first_block_index, log_0_first_block_index);
        assert_eq!(last_block_index, log_0_last_block_index);
        assert_eq!(log_data, log_0_data);
    }
    {
        // test read_block for block_1
        let result = read_log(&mut storage, log_1_first_block_index);
        assert_eq!(result.is_ok(), true);
        let (first_block_index, last_block_index, log_data) = result.unwrap();
        assert_eq!(first_block_index, log_1_first_block_index);
        assert_eq!(last_block_index, log_1_last_block_index);
        assert_eq!(log_data, log_1_data);
    }
    // append logs in storage
    // - log 0
    let log_0_append_data = vec![8_u8, 7_u8, 6_u8, 5_u8, 4_u8, 3_u8, 2_u8, 1_u8];
    let result = append_log(&mut storage, log_0_first_block_index, &log_0_append_data);
    assert_eq!(result.is_ok(), true);
    let log_0_last_block_index = result.unwrap();
    let mut log_0_data = log_0_data;
    log_0_data.extend_from_slice(&log_0_append_data);
    {
        // test read_block for block_0
        let result = read_log(&mut storage, log_0_first_block_index);
        assert_eq!(result.is_ok(), true);
        let (first_block_index, last_block_index, log_data) = result.unwrap();
        assert_eq!(first_block_index, log_0_first_block_index);
        assert_eq!(last_block_index, log_0_last_block_index);
        assert_eq!(log_data, log_0_data);
    }
    // - log 1
    let log_1_append_data = vec![
        16_u8, 15_u8, 14_u8, 13_u8, 12_u8, 11_u8, 10_u8, 9_u8, 8_u8, 7_u8, 6_u8, 5_u8, 4_u8, 3_u8,
        2_u8, 1_u8,
    ];
    let result = append_log(&mut storage, log_1_first_block_index, &log_1_append_data);
    assert_eq!(result.is_ok(), true);
    let log_1_last_block_index = result.unwrap();
    let mut log_1_data = log_1_data;
    log_1_data.extend_from_slice(&log_1_append_data);
    {
        // test read_block for block_1
        let result = read_log(&mut storage, log_1_first_block_index);
        assert_eq!(result.is_ok(), true);
        let (first_block_index, last_block_index, log_data) = result.unwrap();
        assert_eq!(first_block_index, log_1_first_block_index);
        assert_eq!(last_block_index, log_1_last_block_index);
        assert_eq!(log_data, log_1_data);
    }
    // - log 2
    let log_2_append_data = vec![
        35_u8, 34_u8, 33_u8, 32_u8, 31_u8, 30_u8, 29_u8, 28_u8, 27_u8, 26_u8, 25_u8, 24_u8, 23_u8,
        22_u8, 21_u8, 20_u8, 19_u8, 18_u8, 17_u8, 16_u8, 15_u8, 14_u8, 13_u8, 12_u8, 11_u8, 10_u8,
        9_u8, 8_u8, 7_u8, 6_u8, 5_u8, 4_u8, 3_u8, 2_u8, 1_u8,
    ];
    let result = append_log(&mut storage, log_2_first_block_index, &log_2_append_data);
    assert_eq!(result.is_ok(), true);
    let log_2_last_block_index = result.unwrap();
    let mut log_2_data = log_2_data;
    log_2_data.extend_from_slice(&log_2_append_data);
    {
        // test read_block for block_2
        let result = read_log(&mut storage, log_2_first_block_index);
        assert_eq!(result.is_ok(), true);
        let (first_block_index, last_block_index, log_data) = result.unwrap();
        assert_eq!(first_block_index, log_2_first_block_index);
        assert_eq!(last_block_index, log_2_last_block_index);
        assert_eq!(log_data, log_2_data);
    }
    {
        // test read_block for block_0
        let result = read_log(&mut storage, log_0_first_block_index);
        assert_eq!(result.is_ok(), true);
        let (first_block_index, last_block_index, log_data) = result.unwrap();
        assert_eq!(first_block_index, log_0_first_block_index);
        assert_eq!(last_block_index, log_0_last_block_index);
        assert_eq!(log_data, log_0_data);
    }
    {
        // test read_block for block_1
        let result = read_log(&mut storage, log_1_first_block_index);
        assert_eq!(result.is_ok(), true);
        let (first_block_index, last_block_index, log_data) = result.unwrap();
        assert_eq!(first_block_index, log_1_first_block_index);
        assert_eq!(last_block_index, log_1_last_block_index);
        assert_eq!(log_data, log_1_data);
    }

    remove_dir_contents(tmp_dir_path);
}
