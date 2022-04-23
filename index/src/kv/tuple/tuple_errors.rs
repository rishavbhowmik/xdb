use util::error::{Error, ErrorType};

pub fn from_byte_cursor_invalid_crud() -> Error {
    Error::new(ErrorType::Unexpected, "invalid crud", None)
}

pub fn index_crud_from_cursor_invalid_eof_at_crud() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "index_crud_from_cursor_invalid_eof_at_crud",
        Some("Missing enough bytes to read crud".to_string()),
    )
}

pub fn index_crud_from_cursor_invalid_bytes() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_index_crud_from_cursor_invalid_bytes",
        Some("The crud is invalid because the bytes were not read.".to_string()),
    )
}

pub fn index_crud_from_cursor_invalid_crud() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_index_crud_from_cursor_invalid_crud",
        Some("The crud is invalid because the crud was not read.".to_string()),
    )
}

pub fn key_from_cursor_invalid_eof_at_key_len() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_key_from_cursor_invalid_end_of_bytes",
        Some("The key is invalid because the end of bytes was reached, but the key length was not read.".to_string()),
    )
}

pub fn key_from_cursor_invalid_bytes_at_key_len() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_key_from_cursor_invalid_bytes",
        Some("The key is invalid because the bytes were not read.".to_string()),
    )
}

pub fn key_from_cursor_invalid_eof_at_key_data() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_key_from_cursor_invalid_end_of_bytes",
        Some("The key is invalid because the end of bytes was reached, but the key data was not read.".to_string()),
    )
}

pub fn key_from_cursor_invalid_bytes_at_key_data() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_key_from_cursor_invalid_bytes",
        Some("The key is invalid because the bytes were not read.".to_string()),
    )
}

pub fn value_from_cursor_invalid_eof_at_value_len() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_value_from_cursor_invalid_end_of_bytes",
        Some("The value is invalid because the end of bytes was reached, but the value length was not read.".to_string()),
    )
}

pub fn value_from_cursor_invalid_bytes_at_value_len() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_value_from_cursor_invalid_bytes",
        Some("The value is invalid because the bytes were not read.".to_string()),
    )
}

pub fn value_from_cursor_invalid_eof_at_value_data() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_value_from_cursor_invalid_end_of_bytes",
        Some("The value is invalid because the end of bytes was reached, but the value data was not read.".to_string()),
    )
}

pub fn value_from_cursor_invalid_bytes_at_value_data() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_value_from_cursor_invalid_bytes",
        Some("The value is invalid because the bytes were not read.".to_string()),
    )
}
