use util::error::{Error, ErrorType};

pub fn from_bytes_parse_key_length_invalid_eof() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_from_bytes_parse_key_length_invalid_end_of_bytes",
        Some(format!("The key length is invalid because the end of bytes was reached, but the key length was not read.")),
    )
}

pub fn from_bytes_parse_key_length_invalid_bytes() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "kv_from_bytes_parse_key_length_invalid_bytes",
        None,
    )
}

pub fn from_bytes_parse_key_data_invalid_eof() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_from_bytes_parse_key_data_invalid_end_of_bytes",
        Some(format!("The key data is invalid because the end of bytes was reached, but the key data was not read.")),
    )
}

pub fn from_bytes_parse_key_data_invalid_bytes() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "kv_from_bytes_parse_key_data_invalid_bytes",
        None,
    )
}

pub fn from_bytes_parse_value_length_invalid_eof() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_from_bytes_parse_value_length_invalid_end_of_bytes",
        Some(format!("The value length is invalid because the end of bytes was reached, but the value length was not read.")),
    )
}

pub fn from_bytes_parse_value_length_invalid_bytes() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "kv_from_bytes_parse_value_length_invalid_bytes",
        None,
    )
}

pub fn from_bytes_parse_value_data_invalid_eof() -> Error {
    Error::new(
        ErrorType::Critical,
        "kv_from_bytes_parse_value_data_invalid_end_of_bytes",
        None,
    )
}

pub fn from_bytes_parse_value_data_invalid_bytes() -> Error {
    Error::new(
        ErrorType::Unexpected,
        "kv_from_bytes_parse_value_data_invalid_bytes",
        None,
    )
}
