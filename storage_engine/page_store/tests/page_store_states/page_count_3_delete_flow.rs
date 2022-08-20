use crate::page_store_states::PageStoreSampleState;

pub fn page_len_4_soft_delete_0_1_2() -> [PageStoreSampleState; 4] {
    // State 0
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
    let state_0 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 1 - soft delete page 0
    let page_store = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={0} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        0, // Page payload size 0-byte - 1 byte representation
        1, 2, 3, // Page payload - no change on soft delete
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![], vec![1, 2, 3], vec![1, 2, 3]];
    let empty_page_indexes = vec![0];
    let state_1 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 2 - soft delete page 1
    let page_store = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={0, 1} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        0, // Page payload size 0-byte - 1 byte representation
        1, 2, 3, // Page payload - no change on soft delete
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        0, // Page payload size 0-byte - 1 byte representation
        1, 2, 3, // Page payload - no change on soft delete
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![], vec![], vec![1, 2, 3]];
    let empty_page_indexes = vec![0, 1];
    let state_2 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 3 - soft delete page 2
    let page_store = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={0, 1, 2} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        0, // Page payload size 0-byte - 1 byte representation
        1, 2, 3, // Page payload - no change on soft delete
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        0, // Page payload size 0-byte - 1 byte representation
        1, 2, 3, // Page payload - no change on soft delete
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        0, // Page payload size 0-byte - 1 byte representation
        1, 2, 3, // Page payload - no change on soft delete
    ];
    let pages = vec![vec![], vec![], vec![]];
    let empty_page_indexes = vec![0, 1, 2];
    let state_3 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    [state_0, state_1, state_2, state_3]
}

pub fn page_len_4_hard_delete_0_1_2() -> [PageStoreSampleState; 4] {
    // State 0
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
    let state_0 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 1 - hard delete page 0
    let page_store = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={0} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        0, // Page payload size 0-byte - 1 byte representation
        0, 0, 0, // Page payload - hard delete
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![], vec![1, 2, 3], vec![1, 2, 3]];
    let empty_page_indexes = vec![0];
    let state_1 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 2 - hard delete page 1
    let page_store = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={0, 1} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        0, // Page payload size 0-byte - 1 byte representation
        0, 0, 0, // Page payload - hard delete
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        0, // Page payload size 0-byte - 1 byte representation
        0, 0, 0, // Page payload - hard delete
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        3, // Page payload size 3-byte - 1 byte representation
        1, 2, 3, // Page payload
    ];
    let pages = vec![vec![], vec![], vec![1, 2, 3]];
    let empty_page_indexes = vec![0, 1];
    let state_2 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 3 - hard delete page 2
    let page_store = "PageStore { page_count=3, page_size_type=u8, page_len=4, page_capacity=3, empty_page_index_set={0, 1, 2} }";
    let file_data = vec![
        // PageStore header
        4, 0, 0, 0, 0, 0, 0, 0, // page length 4-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        0, // Page payload size 0-byte - 1 byte representation
        0, 0, 0, // Page payload - hard delete
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        0, // Page payload size 0-byte - 1 byte representation
        0, 0, 0, // Page payload - hard delete
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        0, // Page payload size 0-byte - 1 byte representation
        0, 0, 0, // Page payload - hard delete
    ];
    let pages = vec![vec![], vec![], vec![]];
    let empty_page_indexes = vec![0, 1, 2];
    let state_3 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    [state_0, state_1, state_2, state_3]
}

pub fn page_len_300_soft_delete_2_1_0() -> [PageStoreSampleState; 4] {
    // State 0
    let page_store = "PageStore { page_count=3, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![80, 0],  // Page payload size 50-byte - 2 byte representation
        vec![7; 80],  // Page payload
        vec![0; 218], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![180, 0], // Page payload size 50-byte - 2 byte representation
        vec![7; 180], // Page payload
        vec![0; 118], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![24, 1],  // Page payload size 50-byte - 2 byte representation
        vec![7; 280], // Page payload
        vec![0; 18],  // Remaining bytes
    ]
    .concat();
    let pages = vec![vec![7; 80], vec![7; 180], vec![7; 280]];
    let empty_page_indexes = vec![];
    let state_0 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 1 - soft delete page 2
    let page_store = "PageStore { page_count=3, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={2} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![80, 0],  // Page payload size 50-byte - 2 byte representation
        vec![7; 80],  // Page payload
        vec![0; 218], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![180, 0], // Page payload size 50-byte - 2 byte representation
        vec![7; 180], // Page payload
        vec![0; 118], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![7; 280], // Page payload - no change on soft delete
        vec![0; 18],  // Remaining bytes
    ]
    .concat();
    let pages = vec![vec![7; 80], vec![7; 180], vec![]];
    let empty_page_indexes = vec![2];
    let state_1 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 2 - soft delete page 1
    let page_store = "PageStore { page_count=3, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={1, 2} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![80, 0],  // Page payload size 50-byte - 2 byte representation
        vec![7; 80],  // Page payload
        vec![0; 218], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![7; 180], // Page payload - no change on soft delete
        vec![0; 118], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![7; 280], // Page payload
        vec![0; 18],  // Remaining bytes
    ]
    .concat();
    let pages = vec![vec![7; 80], vec![], vec![]];
    let empty_page_indexes = vec![1, 2];
    let state_2 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 3 - soft delete page 0
    let page_store = "PageStore { page_count=3, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={0, 1, 2} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![7; 80],  // Page payload - no change on soft delete
        vec![0; 218], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![7; 180], // Page payload - no change on soft delete
        vec![0; 118], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![7; 280], // Page payload
        vec![0; 18],  // Remaining bytes
    ]
    .concat();
    let pages = vec![vec![], vec![], vec![]];
    let empty_page_indexes = vec![0, 1, 2];
    let state_3 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    [state_0, state_1, state_2, state_3]
}

pub fn page_len_300_hard_delete_2_1_0() -> [PageStoreSampleState; 4] {
    // State 0
    let page_store = "PageStore { page_count=3, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![80, 0],  // Page payload size 50-byte - 2 byte representation
        vec![7; 80],  // Page payload
        vec![0; 218], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![180, 0], // Page payload size 50-byte - 2 byte representation
        vec![7; 180], // Page payload
        vec![0; 118], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![24, 1],  // Page payload size 50-byte - 2 byte representation
        vec![7; 280], // Page payload
    ]
    .concat();
    let pages = vec![vec![7; 80], vec![7; 180], vec![7; 280]];
    let empty_page_indexes = vec![];
    let state_0 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 1 - hard delete page 2
    let page_store = "PageStore { page_count=3, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={2} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![80, 0],  // Page payload size 50-byte - 2 byte representation
        vec![7; 80],  // Page payload
        vec![0; 218], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![180, 0], // Page payload size 50-byte - 2 byte representation
        vec![7; 180], // Page payload
        vec![0; 118], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![0; 298], // Remaining bytes - Hard delete ovewrites entire page with 0s
    ]
    .concat();
    let pages = vec![vec![7; 80], vec![7; 180], vec![]];
    let empty_page_indexes = vec![2];
    let state_1 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 2 - hard delete page 1
    let page_store = "PageStore { page_count=3, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={1, 2} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![80, 0],  // Page payload size 50-byte - 2 byte representation
        vec![7; 80],  // Page payload
        vec![0; 218], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![0; 298], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![0; 298], // Remaining bytes - Hard delete ovewrites entire page with 0s
    ]
    .concat();
    let pages = vec![vec![7; 80], vec![], vec![]];
    let empty_page_indexes = vec![1, 2];
    let state_2 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    // State 3 - hard delete page 0
    let page_store = "PageStore { page_count=3, page_size_type=u16, page_len=300, page_capacity=298, empty_page_index_set={0, 1, 2} }";
    let file_data = vec![
        // PageStore header
        vec![44, 1, 0, 0, 0, 0, 0, 0], // page length 300-byte - 8 bytes representation
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 0
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![0; 298], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 1
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![0; 298], // Remaining bytes
        // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
        // PageIndex: 2
        vec![0, 0],   // Page payload size 0-byte - 2 byte representation
        vec![0; 298], // Remaining bytes - Hard delete ovewrites entire page with 0s
    ]
    .concat();
    let pages = vec![vec![], vec![], vec![]];
    let empty_page_indexes = vec![0, 1, 2];
    let state_3 = PageStoreSampleState {
        fmt: page_store,
        file_data,
        pages,
        empty_page_indexes,
    };

    [state_0, state_1, state_2, state_3]
}
