use logchain::{append_log, create_log, delete_log, make_segment_payload_list};

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
fn make_segment_payload_list_new_storage() {}

fn make_segment_payload_list_existing_storage() {}

fn create_log_new_storage() {}

fn create_log_existing_storage() {}

fn append_log_new_storage() {}

fn append_log_existing_storage() {}

fn delete_log_new_storage() {}

fn delete_log_existing_storage() {}
