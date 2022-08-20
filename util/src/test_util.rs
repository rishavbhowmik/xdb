/// Returns (temp_dir_path, temp_file_path)
pub fn make_temp_dir_n_file(file_name: &str) -> (String, String) {
    // // let tmp_file_path = "./tmp/storage_open_new_file.hex";
    let tmp_dir_path = tempfile::tempdir().unwrap().into_path();
    let tmp_file_path: std::path::PathBuf = [
        tmp_dir_path.to_str().unwrap().to_string(),
        String::from(file_name),
    ]
    .iter()
    .collect();
    (
        tmp_dir_path.to_str().unwrap().to_string(),
        tmp_file_path.to_str().unwrap().to_string(),
    )
}

pub fn read_file(file_path: &str) -> Vec<u8> {
    use std::fs::read;
    use std::path::Path;
    let read_result = read(Path::new(file_path));
    match read_result {
        Ok(data) => data,
        Err(e) => panic!("{:?}", e),
    }
}

pub fn write_file(file_path: &str, data: &[u8]) {
    use std::fs::write;
    use std::path::Path;
    let write_result = write(Path::new(file_path), data);
    match write_result {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
}

pub fn rmdir_recursive(dir_path: std::path::PathBuf) {
    use std::fs::{read_dir, remove_dir, remove_file};
    for entry in read_dir(dir_path.clone()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.file_type().unwrap().is_dir() {
            rmdir_recursive(dir_path.clone());
            remove_dir(path).unwrap();
        } else {
            remove_file(path).unwrap();
        }
    }
    remove_dir(dir_path).unwrap();
}
