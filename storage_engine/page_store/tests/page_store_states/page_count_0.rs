use crate::page_store_states::PageStoreSampleState;

pub fn page_len_4() -> PageStoreSampleState {
    let page_store = "PageStore { page_count=0, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header - page length 4 - 8 bytes representation
        4, 0, 0, 0, 0, 0, 0, 0,
    ];
    let pages = vec![];
    let empty_page_indexes = vec![];
    PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_255() -> PageStoreSampleState {
    let page_store = "PageStore { page_count=0, page_size_type=u8, page_len=255, page_capacity=254, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header - page length 255 - 8 bytes representation
        255, 0, 0, 0, 0, 0, 0, 0,
    ];
    let pages = vec![];
    let empty_page_indexes = vec![];
    PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_256() -> PageStoreSampleState {
    let page_store = "PageStore { page_count=0, page_size_type=u16, page_len=256, page_capacity=254, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header - page length 256 - 8 bytes representation
        0, 1, 0, 0, 0, 0, 0, 0,
    ];
    let pages = vec![];
    let empty_page_indexes = vec![];
    PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_65535() -> PageStoreSampleState {
    let page_store = "PageStore { page_count=0, page_size_type=u16, page_len=65535, page_capacity=65533, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header - page length 65535 - 8 bytes representation
        255, 255, 0, 0, 0, 0, 0, 0,
    ];
    let pages = vec![];
    let empty_page_indexes = vec![];
    PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_65536() -> PageStoreSampleState {
    let page_store = "PageStore { page_count=0, page_size_type=u32, page_len=65536, page_capacity=65532, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header - page length of 65536 - 8 bytes representation
        0, 0, 1, 0, 0, 0, 0, 0,
    ];
    let pages = vec![];
    let empty_page_indexes = vec![];
    PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_4294967295() -> PageStoreSampleState {
    let page_store = "PageStore { page_count=0, page_size_type=u32, page_len=4294967295, page_capacity=4294967291, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header - page length of 4294967295 - 8 bytes representation
        255, 255, 255, 255, 0, 0, 0, 0,
    ];
    let pages = vec![];
    let empty_page_indexes = vec![];
    PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_4294967296() -> PageStoreSampleState {
    let page_store = "PageStore { page_count=0, page_size_type=u64, page_len=4294967296, page_capacity=4294967288, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header - page length of 4294967296 - 8 bytes representation
        0, 0, 0, 0, 1, 0, 0, 0,
    ];
    let pages = vec![];
    let empty_page_indexes = vec![];
    PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_18446744073709551615() -> PageStoreSampleState {
    let page_store = "PageStore { page_count=0, page_size_type=u64, page_len=18446744073709551615, page_capacity=18446744073709551607, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header - page length of 18446744073709551615 - 8 bytes representation
        255, 255, 255, 255, 255, 255, 255, 255,
    ];
    let pages = vec![];
    let empty_page_indexes = vec![];
    PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    }
}
