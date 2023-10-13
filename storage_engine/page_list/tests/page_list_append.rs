use std::sync::Mutex;

use util::test_util::{make_temp_dir_n_file, read_file, rmdir_recursive};
use page_store::PageStore;
use page_list::PageListController;

#[test]
fn page_store_4_attach_plc() {
    // Test PageStore::open_new with page length of 8 bytes
    let (tmp_dir_path, tmp_file_path) = make_temp_dir_n_file("page_store_open_new_pl_4.hex");

    const PAGE_LENGTH: usize = 4;

    let page_store_result = PageStore::open_new(&tmp_file_path, PAGE_LENGTH);
    let page_store = page_store_result.unwrap();

    let page_store_mutex = Mutex::from(page_store);

    let page_list_controller_result = PageListController::new(page_store_mutex, vec![
    ]);

    assert!(page_list_controller_result.is_ok());
    assert_eq!(read_file(&tmp_file_path), [4,0,0,0,0,0,0,0]);

    // let page_list_controller = page_list_controller_result.unwrap();

    // page_list_controller.append_list(0, [1,2,3].to_vec()).unwrap();

    // println!("{:?}", read_file(&tmp_file_path));

    // Clear the temp dir
    rmdir_recursive(std::path::PathBuf::from(&tmp_dir_path));
}
