use page_store::PageStore;
use util::test_util::{make_temp_dir_n_file, read_file, rmdir_recursive, write_file};

mod page_store_states;

#[test]
fn page_store_soft_delete_page_0_1_2_pl_4() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_soft_delete_page_0_1_2_pl_4.hex");

    let page_store_states =
        page_store_states::page_count_3_delete_flow::page_len_4_soft_delete_0_1_2();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_states[0].file_data);

    // Open page_store
    let mut page_store = PageStore::open_existing(&tmp_file_path).unwrap();

    // Soft delete page 0
    let page_index = 0;
    let result = page_store.delete_page(page_index, false);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[1].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[1].file_data);

    // Soft delete page 1
    let page_index = 1;
    let result = page_store.delete_page(page_index, false);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[2].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[2].file_data);

    // Soft delete page 2
    let page_index = 2;
    let result = page_store.delete_page(page_index, false);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[3].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[3].file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_hard_delete_page_0_1_2_pl_4() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_hard_delete_page_0_1_2_pl_4.hex");

    let page_store_states =
        page_store_states::page_count_3_delete_flow::page_len_4_hard_delete_0_1_2();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_states[0].file_data);

    // Open page_store
    let mut page_store = PageStore::open_existing(&tmp_file_path).unwrap();

    // Hard delete page 0
    let page_index = 0;
    let result = page_store.delete_page(page_index, true);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[1].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[1].file_data);

    // Hard delete page 1
    let page_index = 1;
    let result = page_store.delete_page(page_index, true);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[2].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[2].file_data);

    // Hard delete page 2
    let page_index = 2;
    let result = page_store.delete_page(page_index, true);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[3].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[3].file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_soft_delete_page_2_1_0_pl_300() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_soft_delete_page_2_1_0_pl_300.hex");

    let page_store_states =
        page_store_states::page_count_3_delete_flow::page_len_300_soft_delete_2_1_0();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_states[0].file_data);

    // Open page_store
    let mut page_store = PageStore::open_existing(&tmp_file_path).unwrap();

    // Soft delete page 2
    let page_index = 2;
    let result = page_store.delete_page(page_index, false);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[1].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[1].file_data);

    // Soft delete page 1
    let page_index = 1;
    let result = page_store.delete_page(page_index, false);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[2].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[2].file_data);

    // Soft delete page 0
    let page_index = 0;
    let result = page_store.delete_page(page_index, false);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[3].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[3].file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_hard_delete_page_2_1_0_pl_300() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_hard_delete_page_2_1_0_pl_300.hex");

    let page_store_states =
        page_store_states::page_count_3_delete_flow::page_len_300_hard_delete_2_1_0();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_states[0].file_data);

    // Open page_store
    let mut page_store = PageStore::open_existing(&tmp_file_path).unwrap();

    // Hard delete page 2
    let page_index = 2;
    let result = page_store.delete_page(page_index, true);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[1].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[1].file_data);

    // Hard delete page 1
    let page_index = 1;
    let result = page_store.delete_page(page_index, true);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[2].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[2].file_data);

    // Hard delete page 0
    let page_index = 0;
    let result = page_store.delete_page(page_index, true);
    assert!(result.is_ok());
    assert_eq!(format!("{:?}", page_store), page_store_states[3].fmt);
    assert_eq!(read_file(&tmp_file_path), page_store_states[3].file_data);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}
