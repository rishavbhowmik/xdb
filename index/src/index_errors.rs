use util::error::{Error, ErrorType};

pub fn btree_index_from_bytes_empty_tupple() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "btree_index_from_bytes_empty_tupple",
        None,
    )
}

pub fn btree_index_from_bytes_key_to_delete() -> Error {
    Error::new(
        ErrorType::Critical,
        "btree_index_from_bytes_key_to_delete",
        None,
    )
}

pub fn btree_index_from_bytes_key_to_insert() -> Error {
    Error::new(
        ErrorType::Critical,
        "btree_index_from_bytes_key_to_insert",
        None,
    )
}

pub fn btree_index_from_bytes_value_to_insert() -> Error {
    Error::new(
        ErrorType::Critical,
        "btree_index_from_bytes_value_to_insert",
        None,
    )
}

pub fn btree_index_from_bytes_unique_remove() -> Error {
    Error::new(
        ErrorType::Critical,
        "btree_index_from_bytes_unique_remove",
        None,
    )
}

pub fn btree_index_from_bytes_key_to_remove() -> Error {
    Error::new(
        ErrorType::Critical,
        "btree_index_from_bytes_key_to_remove",
        None,
    )
}

pub fn btree_index_from_bytes_value_to_remove() -> Error {
    Error::new(
        ErrorType::Critical,
        "btree_index_from_bytes_value_to_remove",
        None,
    )
}

pub fn btree_index_from_bytes_crud_not_supported() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "btree_index_from_bytes_crud_not_supported",
        None,
    )
}
