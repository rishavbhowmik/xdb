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
    let block_len = 8;
    let storage_result = Storage::new(String::from(tmp_file_path), block_len);
    assert_eq!(storage_result.is_ok(), true);
    let mut storage = storage_result.unwrap();
    let expected = fetch_state("on_create.hex");
    let actual = read_full_file(tmp_file_path);
    assert_eq!(actual, expected);

    // create log 0
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
    assert_eq!(
        segment_list[0].1.len(),
        SIZE_OF_BLOCK_INDEX + block_len as usize
    );
    assert_eq!(segment_list[1].0, 1);
    assert_eq!(
        segment_list[1].1.len(),
        SIZE_OF_BLOCK_INDEX + block_len as usize
    );
    remove_dir_contents(std::path::PathBuf::from(tmp_dir_path));
}

fn make_segment_payload_list_existing_storage() {}

fn create_log_new_storage() {}

fn create_log_existing_storage() {}

fn append_log_new_storage() {}

fn append_log_existing_storage() {}

fn delete_log_new_storage() {}

fn delete_log_existing_storage() {}
