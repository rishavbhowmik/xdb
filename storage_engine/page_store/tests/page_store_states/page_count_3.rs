use crate::page_store_states::PageStoreSampleState;

pub fn page_len_4_page_del_0() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        1, // Page payload size 1-byte - 1 byte representation
        1, // Page payload
        0, 0, // Remaining 2 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        2, // Page payload size 2-byte - 1 byte representation
        1, 2, // Page payload
        0, // Remaining 1 byte
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        2, // Page payload size 2-byte - 1 byte representation
        1,
        2, // Page payload
           // Remaining 0 bytes - assuming last write on the page, is the longest write on the page in it's lifetime
    ];
    let pages = vec![vec![1], vec![1, 2], vec![1, 2]];
    let empty_page_indexes = vec![]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_4_page_del_1() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={1} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4 - 8 bytes representation
        // PageIndex: 0
        3, // Page payload size 3-byte size - 1 byte representation
        1, 2, 3, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        0, // Page payload size 0-byte - deleted page - 1 byte representation
        0, 0, 0, // Remaining 3 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        3, // Page payload size 3-byte size - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![1, 2, 3], vec![], vec![1, 2, 3]];
    let empty_page_indexes = vec![1]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_4_page_del_2() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={0, 2} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4 - 8 bytes representation
        // PageIndex: 0
        0, // Page payload size 0-byte - deleted page - 1 byte representation
        0, 0, 0, // Remaining 3 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        2, // Page payload size 2-byte - 1 byte representation
        1, 2, // Page payload
        0, // Remaining 1 byte
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        0, // Page payload size 0-byte - deleted page - 1 byte representation
        0,
        0, // Remaining 2 bytes - assuming last write on the page was with payload of length 2-byte only
    ];
    let pages = vec![vec![], vec![1, 2], vec![]];
    let empty_page_indexes = vec![0, 2]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_4_page_del_3() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={0, 1, 2} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4 - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        0, // Page payload size 0-byte - deleted page - 1 byte representation
        0, 0, 0, // Remaining 3 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        0, // Page payload size 0-byte - deleted page - 1 byte representation
        0, 0, 0, // Remaining 3 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        0, // Page payload size 0-byte - deleted page - 1 byte representation
        0, 0, 0, // Remaining 3 bytes
    ];
    let pages = vec![vec![], vec![], vec![]];
    let empty_page_indexes = vec![0, 1, 2]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_260_page_del_0() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u16, page_len=260, page_capacity=258, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        vec![4, 1, 0, 0, 0, 0, 0, 0], // page length 260-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![1, 0],   // Page payload size 1-byte - 2 bytes representation
        vec![1],      // Page payload
        vec![0; 257], // Remaining 257 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![2, 0],   // Page payload size 2-byte - 2 bytes representation
        vec![1, 2],   // Page payload
        vec![0; 256], // Remaining 256 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![3, 0],    // Page payload size 3-byte - 3 bytes representation
        vec![1, 2, 3], // Page payload
        vec![0, 255],  // Remaining 255 bytes
    ]
    .concat();
    let pages = vec![vec![1], vec![1, 2], vec![1, 2, 3]];
    let empty_page_indexes = vec![]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_260_page_del_1() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u16, page_len=260, page_capacity=258, empty_page_index_set={0} }";
    let file_data = vec![
        // PageStore header
        vec![4, 1, 0, 0, 0, 0, 0, 0], // page length 260-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![0, 0],   // Page payload size 0-byte - deleted page - 2 byte representation
        vec![0; 258], // Remaining 258 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![2, 0],   // Page payload size 2-byte - 2 bytes representation
        vec![1, 2],   // Page payload
        vec![0; 256], // Remaining 256 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![3, 0],    // Page payload size 3-byte - 3 bytes representation
        vec![1, 2, 3], // Page payload
        vec![0; 255],  // Remaining 255 bytes
    ]
    .concat();
    let pages = vec![vec![], vec![1, 2], vec![1, 2, 3]];
    let empty_page_indexes = vec![0]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_260_page_del_2() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u16, page_len=260, page_capacity=258, empty_page_index_set={0, 1} }";
    let file_data = [
        // PageStore header
        vec![4, 1, 0, 0, 0, 0, 0, 0], // page length 260-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![0, 0],   // Page payload size 0-byte - deleted page - 2 byte representation
        vec![0; 258], // Remaining 258 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![0, 0],   // Page payload size 0-byte - deleted page - 2 byte representation
        vec![0; 258], // Remaining 258 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![3, 0], // Page payload size 3-byte - 3 bytes representation
        vec![1, 2, 3], // Page payload
                    // Remaining 0 bytes - assuming last write on the page, is the longest write on the page in it's lifetime
    ]
    .concat();
    let pages = vec![vec![], vec![], vec![1, 2, 3]];
    let empty_page_indexes = vec![0, 1]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_260_page_del_3() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u16, page_len=260, page_capacity=258, empty_page_index_set={0, 1, 2} }";
    let file_data = [
        // PageStore header
        vec![4, 1, 0, 0, 0, 0, 0, 0], // page length 260-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![0, 0],   // Page payload size 0-byte - deleted page - 2 byte representation
        vec![0; 258], // Remaining 258 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![0, 0],   // Page payload size 0-byte - deleted page - 2 byte representation
        vec![0; 256], // Remaining 256 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![0, 0],    // Page payload size 0-byte - deleted page - 2 byte representation
        vec![0, 0, 0], // Remaining 3 bytes - assuming 3 bytes, is the longest write on the page in it's lifetime
    ]
    .concat();
    let pages = vec![vec![], vec![], vec![]];
    let empty_page_indexes = vec![0, 1, 2]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_65540_page_del_0() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u32, page_len=65540, page_capacity=65536, empty_page_index_set={} }";
    let file_data = [
        // PageStore header
        vec![4, 0, 1, 0, 0, 0, 0, 0], // page length 65540-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![10, 0, 0, 0], // Page payload size 10-byte - 4 bytes representation
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], // Page payload
        vec![0; 65526],    // Remaining 65526 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![10, 0, 0, 0], // Page payload size 10-byte - 4 bytes representation
        vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19], // Page payload
        vec![0; 65526],    // Remaining 65526 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![255, 255, 0, 0], // Page payload size 65535-byte - 4 bytes representation
        vec![99; 65535],      // Page payload size 65535-byte - 65535 bytes representation
                              // No more bytes - assuming 65535 bytes, is the longest write on the page in it's lifetime
    ]
    .concat();
    let pages = vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
        vec![99; 65535],
    ];
    let empty_page_indexes = vec![]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_65540_page_del_1() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u32, page_len=65540, page_capacity=65536, empty_page_index_set={1} }";
    let file_data = [
        // PageStore header
        vec![4, 0, 1, 0, 0, 0, 0, 0], // page length 65540-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![10, 0, 0, 0], // Page payload size 10-byte - 4 bytes representation
        vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19], // Page payload
        vec![0; 65526],    // Remaining 65526 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![0, 0, 0, 0], // Page payload size 0-byte - deleted page - 4 byte representation
        vec![0; 65536],   // Remaining 65536 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![10, 0, 0, 0], // Page payload size 10-byte - 4 bytes representation
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], // Page payload
        vec![8; 65526],    // Remaining 65526 bytes - with footprint of older page data
    ]
    .concat();
    let pages = vec![
        vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
        vec![],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    ];
    let empty_page_indexes = vec![1]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_65540_page_del_2() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u32, page_len=65540, page_capacity=65536, empty_page_index_set={0, 2} }";
    let file_data = [
        // PageStore header
        vec![4, 0, 1, 0, 0, 0, 0, 0], // page length 65540-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![0, 0, 0, 0], // Page payload size 0-byte - deleted page - 4 byte representation
        vec![255; 65536], // Remaining 65536 bytes - with footprint of older page data
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![10, 0, 0, 0], // Page payload size 10-byte - 4 bytes representation
        vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19], // Page payload
        vec![0; 65526],    // Remaining 65526 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![0, 0, 0, 0], // Page payload size 0-byte - deleted page - 4 byte representation
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![0; 65526], // Remaining 65526 bytes - with footprint of older page data
    ]
    .concat();
    let pages = vec![
        vec![],
        vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    ];
    let empty_page_indexes = vec![0, 2]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}

pub fn page_len_65540_page_del_3() -> PageStoreSampleState {
    let fmt = "PageStore { page_count=3, page_size_type=u32, page_len=65540, page_capacity=65536, empty_page_index_set={0, 1, 2} }";
    let file_data = [
        // PageStore header
        vec![4, 0, 1, 0, 0, 0, 0, 0], // page length 65540-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![0, 0, 0, 0], // Page payload size 0-byte - deleted page - 4 byte representation
        vec![0; 65536],   // Remaining 65536 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![0, 0, 0, 0], // Page payload size 0-byte - deleted page - 4 byte representation
        vec![0; 65536],   // Remaining 65536 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![0, 0, 0, 0], // Page payload size 0-byte - deleted page - 4 byte representation
        vec![0; 65536],   // Remaining 65536 bytes
    ]
    .concat();
    let pages = vec![vec![], vec![], vec![]];
    let empty_page_indexes = vec![0, 1, 2]; // All page_index with payload size 0
    PageStoreSampleState {
        fmt,
        file_data,
        pages,
        empty_page_indexes,
    }
}
