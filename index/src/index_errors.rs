use util::error::{Error, ErrorType};

pub fn index_trait_remove_value_not_found() -> Error {
    Error::new(
        ErrorType::Happens,
        "index_trait_remove_value_not_found",
        Some("The value was not found to remove".to_string()),
    )
}

pub fn index_trait_remove_key_not_found() -> Error {
    Error::new(
        ErrorType::Happens,
        "index_trait_remove_key_not_found",
        Some("The key was not found to remove".to_string()),
    )
}

pub fn index_trait_delete_key_not_found() -> Error {
    Error::new(
        ErrorType::Happens,
        "index_trait_delete_key_not_found",
        Some("The key was not found to delete".to_string()),
    )
}

pub fn btree_index_from_bytes_delete_key_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "btree_index_from_bytes_delete_key_not_found",
        Some("The key was not found to delete".to_string()),
    )
}

pub fn btree_index_from_bytes_delete_key_or_value_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "btree_index_from_bytes_delete_key_or_value_not_found",
        Some("The key or value was not found to delete".to_string()),
    )
}

pub fn unique_index_trait_set_key_occupied() -> Error {
    Error::new(
        ErrorType::Happens,
        "unique_index_trait_set_key_occupied",
        Some("Option overwrite set to false, but the key is occupied".to_string()),
    )
}

pub fn unique_btree_index_from_bytes_delete_key_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_btree_index_from_bytes_delete_key_not_found",
        Some("The key was not found to delete".to_string()),
    )
}

pub fn unique_btree_index_from_bytes_insert_key_or_value_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_btree_index_from_bytes_delete_key_or_value_not_found",
        Some("The key or value was not found to delete".to_string()),
    )
}

pub fn unique_btree_index_from_bytes_remove_key_or_value_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_btree_index_from_bytes_remove_key_or_value_not_found",
        Some("The key or value was not found to remove".to_string()),
    )
}

pub fn hash_map_index_from_bytes_delete_key_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "hash_map_index_from_bytes_delete_key_not_found",
        Some("The key was not found to delete".to_string()),
    )
}

pub fn hash_map_index_from_bytes_delete_key_or_value_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "hash_map_index_from_bytes_delete_key_or_value_not_found",
        Some("The key or value was not found to delete".to_string()),
    )
}

pub fn hash_map_index_from_bytes_remove_key_or_value_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "hash_map_index_from_bytes_remove_key_or_value_not_found",
        Some("The key or value was not found to remove".to_string()),
    )
}

pub fn unique_hash_map_index_from_bytes_delete_key_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_hash_map_index_from_bytes_delete_key_not_found",
        Some("The key was not found to delete".to_string()),
    )
}

pub fn unique_hash_map_index_from_bytes_insert_key_or_value_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_hash_map_index_from_bytes_insert_key_or_value_not_found",
        Some("The key or value was not found to insert".to_string()),
    )
}

pub fn unique_hash_map_index_from_bytes_remove_key_or_value_not_found() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_hash_map_index_from_bytes_remove_key_or_value_not_found",
        Some("The key or value was not found to remove".to_string()),
    )
}
