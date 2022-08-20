use page_store::PageStore;
use util::test_util::{make_temp_dir_n_file, read_file, rmdir_recursive};

mod page_store_states;

#[test]
fn page_store_open_new_pl_4() {
    // Test PageStore::open_new with page length of 8 bytes
    let (tmp_dir_path, tmp_file_path) = make_temp_dir_n_file("page_store_open_new_pl_4.hex");

    const PAGE_LENGTH: usize = 4;

    let page_store_result = PageStore::open_new(&tmp_file_path, PAGE_LENGTH);
    let page_store = page_store_result.unwrap();

    // Verify page store state
    let page_store_state = page_store_states::page_count_0::page_len_4();

    assert_eq!(format!("{:?}", page_store), page_store_state.fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_state.file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_open_new_pl_255() {
    // Test PageStore::open_new with page length of 255 bytes
    let (tmp_dir_path, tmp_file_path) = make_temp_dir_n_file("page_store_open_new_pl_255.hex");

    const PAGE_LENGTH: usize = 255;

    let page_store_result = PageStore::open_new(&tmp_file_path, PAGE_LENGTH);
    let page_store = page_store_result.unwrap();

    // Verify page store state
    let page_store_state = page_store_states::page_count_0::page_len_255();

    assert_eq!(format!("{:?}", page_store), page_store_state.fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_state.file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_open_new_pl_256() {
    // Test PageStore::open_new with page length of 256 bytes
    let (tmp_dir_path, tmp_file_path) = make_temp_dir_n_file("page_store_open_new_pl_256.hex");

    const PAGE_LENGTH: usize = 256;

    let page_store_result = PageStore::open_new(&tmp_file_path, PAGE_LENGTH);
    let page_store = page_store_result.unwrap();

    // Verify page store state
    let page_store_state = page_store_states::page_count_0::page_len_256();

    assert_eq!(format!("{:?}", page_store), page_store_state.fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_state.file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_open_new_pl_65535() {
    // Test PageStore::open_new with page length of 65535 bytes
    let (tmp_dir_path, tmp_file_path) = make_temp_dir_n_file("page_store_open_new_pl_65535.hex");

    const PAGE_LENGTH: usize = 65535;

    let page_store_result = PageStore::open_new(&tmp_file_path, PAGE_LENGTH);
    let page_store = page_store_result.unwrap();

    // Verify page store state
    let page_store_state = page_store_states::page_count_0::page_len_65535();

    assert_eq!(format!("{:?}", page_store), page_store_state.fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_state.file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_open_new_pl_65536() {
    // Test PageStore::open_new with page length of 65536 bytes
    let (tmp_dir_path, tmp_file_path) = make_temp_dir_n_file("page_store_open_new_pl_65536.hex");

    const PAGE_LENGTH: usize = 65536;

    let page_store_result = PageStore::open_new(&tmp_file_path, PAGE_LENGTH);
    let page_store = page_store_result.unwrap();

    // Verify page store state
    let page_store_state = page_store_states::page_count_0::page_len_65536();

    assert_eq!(format!("{:?}", page_store), page_store_state.fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_state.file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_open_new_pl_4294967295() {
    // Test PageStore::open_new with page length of 4294967295 bytes
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_open_new_pl_4294967295.hex");

    const PAGE_LENGTH: usize = 4294967295;

    let page_store_result = PageStore::open_new(&tmp_file_path, PAGE_LENGTH);
    let page_store = page_store_result.unwrap();

    // Verify page store state
    let page_store_state = page_store_states::page_count_0::page_len_4294967295();

    assert_eq!(format!("{:?}", page_store), page_store_state.fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_state.file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_open_new_pl_4294967296() {
    // Test PageStore::open_new with page length of 4294967296 bytes
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_open_new_pl_4294967296.hex");

    const PAGE_LENGTH: usize = 4294967296;

    let page_store_result = PageStore::open_new(&tmp_file_path, PAGE_LENGTH);
    let page_store = page_store_result.unwrap();

    // Verify page store state
    let page_store_state = page_store_states::page_count_0::page_len_4294967296();

    assert_eq!(format!("{:?}", page_store), page_store_state.fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_state.file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_open_new_pl_18446744073709551615() {
    // Test PageStore::open_new with page length of 18446744073709551615 bytes
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_open_new_pl_18446744073709551615.hex");

    const PAGE_LENGTH: usize = 18446744073709551615;

    let page_store_result = PageStore::open_new(&tmp_file_path, PAGE_LENGTH);
    let page_store = page_store_result.unwrap();

    // Verify page store state
    let page_store_state = page_store_states::page_count_0::page_len_18446744073709551615();

    assert_eq!(format!("{:?}", page_store), page_store_state.fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_state.file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}
