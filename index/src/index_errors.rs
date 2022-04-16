use util::error::{Error, ErrorType};

pub fn index_trait_remove_value_not_found() -> Error {
    Error::new(
        ErrorType::Happens,
        "index_trait_remove_value_not_found",
        Some("value to remove is not found against the key".to_string()),
    )
}

pub fn index_trait_remove_key_not_found() -> Error {
    Error::new(
        ErrorType::Happens,
        "index_trait_remove_key_not_found",
        Some("key to remove is not found in the index".to_string()),
    )
}

pub fn index_trait_delete_key_not_found() -> Error {
    Error::new(
        ErrorType::Happens,
        "index_trait_delete_key_not_found",
        Some("key to delete is not found in the index".to_string()),
    )
}

pub fn btree_index_from_bytes_delete_key_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "btree_index_from_bytes_delete_key_not_provided",
        Some("key is required to delete the key".to_string()),
    )
}

pub fn btree_index_from_bytes_delete_key_or_value_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "btree_index_from_bytes_delete_key_or_value_not_provided",
        Some("(key, value) is required to delete the KV pair".to_string()),
    )
}

pub fn unique_index_trait_set_key_occupied() -> Error {
    Error::new(
        ErrorType::Happens,
        "unique_index_trait_set_key_occupied",
        Some("Option overwrite set to false and the key is occupied".to_string()),
    )
}

pub fn unique_btree_index_from_bytes_delete_key_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_btree_index_from_bytes_delete_key_not_provided",
        Some("key is required to delete the key".to_string()),
    )
}

pub fn unique_btree_index_from_bytes_insert_key_or_value_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_btree_index_from_bytes_delete_key_or_value_not_provided",
        Some("(key, value) is required to insert the KV pair".to_string()),
    )
}

pub fn unique_btree_index_from_bytes_remove_key_or_value_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_btree_index_from_bytes_remove_key_or_value_not_provided",
        Some("(key, value) is required to remove the KV pair".to_string()),
    )
}

pub fn hash_map_index_from_bytes_delete_key_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "hash_map_index_from_bytes_delete_key_not_provided",
        Some("key is required to delete the key".to_string()),
    )
}

pub fn hash_map_index_from_bytes_delete_key_or_value_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "hash_map_index_from_bytes_delete_key_or_value_not_provided",
        Some("(key, value) is required to delete the KV pair".to_string()),
    )
}

pub fn hash_map_index_from_bytes_remove_key_or_value_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "hash_map_index_from_bytes_remove_key_or_value_not_provided",
        Some("(key, value) is required to remove the KV pair".to_string()),
    )
}

pub fn unique_hash_map_index_from_bytes_delete_key_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_hash_map_index_from_bytes_delete_key_not_provided",
        Some("key is required to delete the key".to_string()),
    )
}

pub fn unique_hash_map_index_from_bytes_insert_key_or_value_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_hash_map_index_from_bytes_insert_key_or_value_not_provided",
        Some("(key, value) is required to insert the KV pair".to_string()),
    )
}

pub fn unique_hash_map_index_from_bytes_remove_key_or_value_not_provided() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "unique_hash_map_index_from_bytes_remove_key_or_value_not_provided",
        Some("(key, value) is required to remove the KV pair".to_string()),
    )
}
