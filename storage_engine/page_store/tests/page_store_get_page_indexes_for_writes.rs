use page_store::PageStore;
use util::test_util::{make_temp_dir_n_file, rmdir_recursive, write_file};

mod page_store_states;

#[test]
fn page_store_get_page_indexes_for_writes_pc_0() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_get_page_indexes_for_writes_pc_0.hex");

    let page_store_state = page_store_states::page_count_0::page_len_4();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store = PageStore::open_existing(&tmp_file_path).unwrap();

    // Get page indexes for writes
    // 0 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(0);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, vec![]);
    // 1 page
    let page_indexes_result = page_store.get_page_indexes_for_writes(1);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, vec![0]);
    // 2 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(2);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, vec![0, 1]);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_get_page_indexes_for_writes_pc_3_pd_0() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_get_page_indexes_for_writes_pc_3_pd_0.hex");

    let page_store_state = page_store_states::page_count_3::page_len_4_page_del_0();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store = PageStore::open_existing(&tmp_file_path).unwrap();

    // Get page indexes for writes
    // 0 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(0);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, vec![]);
    // 1 page
    let page_indexes_result = page_store.get_page_indexes_for_writes(1);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, vec![3]);
    // 2 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(2);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, vec![3, 4]);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_get_page_indexes_for_writes_pc_3_pd_1() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_get_page_indexes_for_writes_pc_3_pd_1.hex");

    let page_store_state = page_store_states::page_count_3::page_len_4_page_del_1();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store = PageStore::open_existing(&tmp_file_path).unwrap();

    // Get page indexes for writes
    // 0 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(0);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, vec![]);
    // 1 page
    let page_indexes_result = page_store.get_page_indexes_for_writes(1);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, page_store_state.empty_page_indexes[..1]);
    // 2 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(2);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(
        page_indexes,
        [page_store_state.empty_page_indexes.clone(), vec![3]].concat()
    );
    // 3 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(3);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(
        page_indexes,
        [page_store_state.empty_page_indexes, vec![3, 4]].concat()
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_get_page_indexes_for_writes_pc_3_pd_2() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_get_page_indexes_for_writes_pc_3_pd_2.hex");

    let page_store_state = page_store_states::page_count_3::page_len_4_page_del_2();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store = PageStore::open_existing(&tmp_file_path).unwrap();

    // Get page indexes for writes
    // 0 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(0);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, vec![]);
    // 1 page
    let page_indexes_result = page_store.get_page_indexes_for_writes(1);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, page_store_state.empty_page_indexes[..1]);
    // 2 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(2);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, page_store_state.empty_page_indexes[..2]);
    // 3 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(3);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(
        page_indexes,
        [page_store_state.empty_page_indexes.clone(), vec![3]].concat()
    );
    // 4 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(4);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(
        page_indexes,
        [page_store_state.empty_page_indexes, vec![3, 4]].concat()
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_get_page_indexes_for_writes_pc_3_pd_3() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_get_page_indexes_for_writes_pc_3_pd_3.hex");

    let page_store_state = page_store_states::page_count_3::page_len_4_page_del_3();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store = PageStore::open_existing(&tmp_file_path).unwrap();

    // Get page indexes for writes
    // 0 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(0);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, vec![]);
    // 1 page
    let page_indexes_result = page_store.get_page_indexes_for_writes(1);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, page_store_state.empty_page_indexes[..1]);
    // 2 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(2);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, page_store_state.empty_page_indexes[..2]);
    // 3 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(3);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(page_indexes, page_store_state.empty_page_indexes[..3]);
    // 4 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(4);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(
        page_indexes,
        [page_store_state.empty_page_indexes.clone(), vec![3]].concat()
    );
    // 5 pages
    let page_indexes_result = page_store.get_page_indexes_for_writes(5);
    assert!(page_indexes_result.is_ok());
    let page_indexes = page_indexes_result.unwrap();
    assert_eq!(
        page_indexes,
        [page_store_state.empty_page_indexes, vec![3, 4]].concat()
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}
