use util::error::{Error, ErrorType};

pub fn btree_index_from_bytes_empty_tupple() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "btree_index_from_bytes_empty_tupple",
        None,
    )
}

pub fn hash_map_from_bytes_empty_tupple() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "hash_map_from_bytes_empty_tupple",
        None,
    )
}
