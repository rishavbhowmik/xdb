use util::error::{Error, ErrorType};

pub fn from_byte_cursor_invalid_crud() -> Error {
    Error::new(ErrorType::Unexpected, "invalid crud", None)
}

pub fn index_crud_from_cursor_invalid_bytes() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_index_crud_from_cursor_invalid_bytes",
        Some(format!(
            "The crud is invalid because the bytes were not read."
        )),
    )
}

pub fn index_crud_from_cursor_invalid_crud() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_index_crud_from_cursor_invalid_crud",
        Some(format!(
            "The crud is invalid because the crud was not read."
        )),
    )
}

pub fn key_from_cursor_invalid_eof_at_key_len() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_key_from_cursor_invalid_end_of_bytes",
        Some(format!("The key is invalid because the end of bytes was reached, but the key length was not read.")),
    )
}

pub fn key_from_cursor_invalid_bytes_at_key_len() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_key_from_cursor_invalid_bytes",
        Some(format!(
            "The key is invalid because the bytes were not read."
        )),
    )
}

pub fn key_from_cursor_invalid_eof_at_key_data() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_key_from_cursor_invalid_end_of_bytes",
        Some(format!("The key is invalid because the end of bytes was reached, but the key data was not read.")),
    )
}

pub fn key_from_cursor_invalid_bytes_at_key_data() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_key_from_cursor_invalid_bytes",
        Some(format!(
            "The key is invalid because the bytes were not read."
        )),
    )
}

pub fn value_from_cursor_invalid_eof_at_value_len() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_value_from_cursor_invalid_end_of_bytes",
        Some(format!("The value is invalid because the end of bytes was reached, but the value length was not read.")),
    )
}

pub fn value_from_cursor_invalid_bytes_at_value_len() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_value_from_cursor_invalid_bytes",
        Some(format!(
            "The value is invalid because the bytes were not read."
        )),
    )
}

pub fn value_from_cursor_invalid_eof_at_value_data() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_value_from_cursor_invalid_end_of_bytes",
        Some(format!("The value is invalid because the end of bytes was reached, but the value data was not read.")),
    )
}

pub fn value_from_cursor_invalid_bytes_at_value_data() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_value_from_cursor_invalid_bytes",
        Some(format!(
            "The value is invalid because the bytes were not read."
        )),
    )
}
