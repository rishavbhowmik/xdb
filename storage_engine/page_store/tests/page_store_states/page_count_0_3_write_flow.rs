use crate::page_store_states::PageStoreSampleState;

pub fn page_len_4() -> [PageStoreSampleState; 4] {
    // State 0
    let page_store = "PageStore { page_count=0, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header - page length 4 - 8 bytes representation
        4, 0, 0, 0, 0, 0, 0, 0,
    ];
    let pages = vec![];
    let empty_page_indexes = vec![];
    let state_0 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    // State 1 - write to page 0
    let page_store = "PageStore { page_count=1, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        3, // Page payload size 2-byte - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![1, 2, 3]];
    let empty_page_indexes = vec![];
    let state_1 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    // State 2 - write to page 1
    let page_store = "PageStore { page_count=2, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![1, 2, 3], vec![1, 2, 3]];
    let empty_page_indexes = vec![];
    let state_2 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    // State 3 - write to page 2
    let page_store = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
    let empty_page_indexes = vec![];
    let state_3 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    [state_0, state_1, state_2, state_3]
}

pub fn page_len_4_page_del_3() -> [PageStoreSampleState; 4] {
    // State 0
    let page_store = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={0, 1, 2} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        0, // Page payload size 0-byte - deleted page - 1 byte representation
        1, 2, 3, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        0, // Page payload size 0-byte - deleted page - 1 byte representation
        1, 2, 3, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        0, // Page payload size 0-byte - deleted page - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![], vec![], vec![]];
    let empty_page_indexes = vec![];
    let state_0 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    // State 1 - write to page 0
    let page_store = "PageStore { page_count=2, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={1, 2} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        3, // Page payload size 3-byte - 1 byte representation
        3, 2, 1, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        0, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        0, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![3, 2, 1], vec![]];
    let empty_page_indexes = vec![];
    let state_1 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    // State 2 - write to page 1
    let page_store = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={2} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        3, // Page payload size 3-byte - 1 byte representation
        3, 2, 1, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        3, // Page payload size 3-byte - 1 byte representation
        3, 2, 1, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        0, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![3, 2, 1], vec![3, 2, 1], vec![]];
    let empty_page_indexes = vec![];
    let state_2 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    // State 3 - write to page 2
    let page_store = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        3, // Page payload size 3-byte - 1 byte representation
        3, 2, 1, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        3, // Page payload size 3-byte - 1 byte representation
        3, 2, 1, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        3, // Page payload size 3-byte - 1 byte representation
        3, 2, 1, // Page payload
    ];
    let pages = vec![vec![3, 2, 1], vec![3, 2, 1], vec![3, 2, 1]];
    let empty_page_indexes = vec![];
    let state_3 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    [state_0, state_1, state_2, state_3]
}

pub fn page_len_300() -> [PageStoreSampleState; 4] {
    // Page count 0
    let page_store = "PageStore { page_count=0, page_size_type=u16, page_len=300, page_capacity=3, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header - page length 300 - 8 bytes representation
        44, 1, 0, 0, 0, 0, 0, 0,
    ];
    let pages = vec![];
    let empty_page_indexes = vec![];
    let state_0 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    // Page count 1
    let page_store = "PageStore { page_count=1, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![250, 0], // Page payload size 250-byte - 1 byte representation
        vec![5; 250], // Page payload
    ]
    .concat();
    let pages = vec![vec![5; 250]];
    let empty_page_indexes = vec![];
    let state_1 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    // Page count 2
    let page_store = "PageStore { page_count=2, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![250, 0], // Page payload size 250-byte - 1 byte representation
        vec![5; 250], // Page payload
        vec![0; 48],  // Remaining 48 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![200, 0], // Page payload size 200-byte - 1 byte representation
        vec![5; 200], // Page payload
    ]
    .concat();
    let pages = vec![vec![5; 250], vec![5; 200]];
    let empty_page_indexes = vec![];
    let state_2 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    // Page count 3
    let page_store = "PageStore { page_count=3, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![250, 0], // Page payload size 250-byte - 1 byte representation
        vec![5; 250], // Page payload
        vec![0; 48],  // Remaining 48 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![200, 0], // Page payload size 200-byte - 1 byte representation
        vec![5; 200], // Page payload
        vec![0; 98],  // Remaining 98 bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![150, 0], // Page payload size 100-byte - 1 byte representation
        vec![5; 150], // Page payload
    ]
    .concat();
    let pages = vec![vec![5; 250], vec![5; 200], vec![5; 150]];
    let empty_page_indexes = vec![];
    let state_3 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };
    [state_0, state_1, state_2, state_3]
}
