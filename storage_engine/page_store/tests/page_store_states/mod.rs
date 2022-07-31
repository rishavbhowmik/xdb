/// Struct for sample page store states for testing.
/// Always use this as immutable entity in the tests.
pub struct PageStoreSampleState {
    pub fmt: &'static str,
    pub file_data: Vec<u8>,
    pub pages: Vec<Vec<u8>>,
    pub empty_page_indexes: Vec<usize>,
}

pub mod page_count_0;
pub mod page_count_0_3_write_flow;
pub mod page_count_3;
pub mod page_count_3_delete_flow;
