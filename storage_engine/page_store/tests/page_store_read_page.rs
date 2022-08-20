use page_store::PageStore;
use util::test_util::{make_temp_dir_n_file, rmdir_recursive, write_file};

mod page_store_states;

#[test]
fn page_store_read_page_pc_0_pl_4() {
    let (tmp_dir_path, tmp_file_path) = make_temp_dir_n_file("page_store_read_page_pc_0_pl_4.hex");

    let page_store_state = page_store_states::page_count_0::page_len_4();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    let page_index = 1;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_read_page_pc_0_pl_256() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_read_page_pc_0_pl_256.hex");

    let page_store_state = page_store_states::page_count_0::page_len_256();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    let page_index = 1;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_read_page_pc_3_pl_4_pd_0() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_read_page_pc_3_pl_4_pd_0.hex");

    let page_store_state = page_store_states::page_count_3::page_len_4_page_del_0();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 1;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 2;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 3;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_read_page_ranged_pc_3_pl_4_pd_0() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_read_page_ranged_pc_3_pl_4_pd_0.hex");

    let page_store_state = page_store_states::page_count_3::page_len_4_page_del_0();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;

    let result = page_store.read_page(page_index, Some(0), Some(0));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, []);

    let result = page_store.read_page(page_index, Some(1), Some(1));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, []);

    let result = page_store.read_page(page_index, Some(0), Some(1));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][0..1]);

    let result = page_store.read_page(page_index, Some(1), Some(2));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, []);

    let result = page_store.read_page(page_index, Some(2), Some(3));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, []);

    let result = page_store.read_page(page_index, None, Some(0));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, []);

    let result = page_store.read_page(page_index, None, Some(1));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][..1]);

    let result = page_store.read_page(page_index, None, Some(2));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][..1]);

    let result = page_store.read_page(page_index, Some(0), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index]);

    let result = page_store.read_page(page_index, Some(1), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, []);

    let result = page_store.read_page(page_index, Some(2), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, []);

    let page_index = 1;

    let result = page_store.read_page(page_index, Some(0), Some(1));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][0..1]);

    let result = page_store.read_page(page_index, Some(0), Some(2));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index]);

    let result = page_store.read_page(page_index, Some(1), Some(2));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][1..2]);

    let result = page_store.read_page(page_index, None, Some(1));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][..1]);

    let result = page_store.read_page(page_index, None, Some(2));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][..2]);

    let result = page_store.read_page(page_index, None, Some(3));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][..2]);

    let result = page_store.read_page(page_index, Some(0), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index]);

    let result = page_store.read_page(page_index, Some(1), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][1..]);

    let result = page_store.read_page(page_index, Some(2), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][2..]);

    let result = page_store.read_page(page_index, Some(3), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, []);

    let page_index = 2;

    let result = page_store.read_page(page_index, Some(0), Some(1));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][0..1]);

    let result = page_store.read_page(page_index, Some(0), Some(2));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index]);

    let result = page_store.read_page(page_index, Some(1), Some(2));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][1..2]);

    let result = page_store.read_page(page_index, None, Some(1));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][..1]);

    let result = page_store.read_page(page_index, None, Some(2));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][..2]);

    let result = page_store.read_page(page_index, None, Some(3));
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][..2]);

    let result = page_store.read_page(page_index, Some(0), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index]);

    let result = page_store.read_page(page_index, Some(1), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][1..]);

    let result = page_store.read_page(page_index, Some(2), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, page_store_state.pages[page_index][2..]);

    let result = page_store.read_page(page_index, Some(3), None);
    assert!(result.is_ok());
    let page_data = result.unwrap();
    assert_eq!(page_data, []);

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_read_page_pc_3_pl_4_pd_1() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_read_page_pc_3_pl_4_pd_1.hex");

    let page_store_state = page_store_states::page_count_3::page_len_4_page_del_1();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 1;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 2;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 3;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_read_page_pc_3_pl_4_pd_2() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_read_page_pc_3_pl_4_pd_2.hex");

    let page_store_state = page_store_states::page_count_3::page_len_4_page_del_2();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 1;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 2;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 3;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_read_page_pc_3_pl_4_pd_3() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_read_page_pc_3_pl_4_pd_3.hex");

    let page_store_state = page_store_states::page_count_3::page_len_4_page_del_3();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 1;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 2;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 3;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_read_page_pc_3_pl_260_pd_0() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_read_page_pc_3_pl_260_pd_0.hex");

    let page_store_state = page_store_states::page_count_3::page_len_260_page_del_0();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 1;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 2;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 3;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_read_page_pc_3_pl_260_pd_1() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_read_page_pc_3_pl_260_pd_1.hex");

    let page_store_state = page_store_states::page_count_3::page_len_260_page_del_1();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 1;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 2;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 3;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_read_page_pc_3_pl_260_pd_2() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_read_page_pc_3_pl_260_pd_2.hex");

    let page_store_state = page_store_states::page_count_3::page_len_260_page_del_2();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 1;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 2;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 3;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}

#[test]
fn page_store_read_page_pc_3_pl_260_pd_3() {
    let (tmp_dir_path, tmp_file_path) =
        make_temp_dir_n_file("page_store_read_page_pc_3_pl_260_pd_3.hex");

    let page_store_state = page_store_states::page_count_3::page_len_260_page_del_3();

    // Initialize tmp file
    write_file(&tmp_file_path, &page_store_state.file_data);

    // Open page_store
    let page_store_result = PageStore::open_existing(&tmp_file_path);
    assert!(page_store_result.is_ok());
    let mut page_store = page_store_result.unwrap();

    // Read pages
    let page_index = 0;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 1;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 2;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_ok());
    let page = result.unwrap();
    assert_eq!(page, page_store_state.pages[page_index]);

    let page_index = 3;
    let result = page_store.read_page(page_index, None, None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code(),
        "page_store_read_page_index_out_of_range"
    );

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}
