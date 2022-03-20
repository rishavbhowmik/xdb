use util::error::{Error, ErrorType};

// .... .... Storage::open_file_writer .... ....

pub fn open_file_writer_open_file(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Happens,
        "open_file_writer_failed_to_open_file",
        Some(format!(
            "Failed to open file for writing, check permissions and path.\n {}",
            io_error
        )),
    )
}

pub fn open_file_reader_open_file(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Happens,
        "open_file_reader_failed_to_open_file",
        Some(format!(
            "Failed to open file for reading, check permissions and path.\n {}",
            io_error
        )),
    )
}

// .... .... Storage::set_storage_header .... ....

pub fn set_storage_header_seek_start(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "set_storage_header_failed_to_seek_write_ptr_to_start",
        Some(format!(
            "Failed to seek write pointer to start of file.\n {}",
            io_error
        )),
    )
}

pub fn set_storage_header_write_header(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Happens,
        "set_storage_header_failed_to_write_header",
        Some(format!(
            "Failed to write header to file, check permissions and disk state.\n {}",
            io_error
        )),
    )
}

pub fn set_storage_header_write_header_success(bytes_written: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "set_storage_header_failed_to_write_header",
        Some(format!(
            "Failed to write header to file, check permissions and path.\n\tBytes Written: {} bytes",
            bytes_written
        )),
    )
}

// .... .... Storage::get_storage_header .... ....

pub fn get_storage_header_seek_start(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "get_storage_header_failed_to_seek_read_ptr_to_start",
        Some(format!(
            "Failed to seek read pointer to start of file.\n {}",
            io_error
        )),
    )
}

pub fn get_storage_header_read_header(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Happens,
        "get_storage_header_failed_to_read_header",
        Some(format!(
            "Failed to read header from file, check permissions and disk state.\n {}",
            io_error
        )),
    )
}

pub fn get_storage_header_read_header_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "get_storage_header_failed_to_read_header",
        Some(format!(
            "Failed to read header from file, check permissions and path.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}

// .... .... Storage::read_storage_block_headers .... ....

pub fn read_storage_block_headers_seek_1st_block_offset(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "read_storage_block_headers_failed_to_seek_read_ptr_to_start",
        Some(format!(
            "Failed to seek read pointer to start of file.\n {}",
            io_error
        )),
    )
}

pub fn read_storage_block_headers_read_block_header(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Happens,
        "read_storage_block_headers_failed_to_read_headers",
        Some(format!(
            "Failed to read headers from file, check permissions and disk state.\n {}",
            io_error
        )),
    )
}

pub fn read_storage_block_headers_read_block_header_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "read_storage_block_headers_failed_to_read_headers",
        Some(format!(
            "Failed to read headers from file, check permissions and path.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}

pub fn read_storage_block_headers_seek_next_block(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "read_storage_block_headers_failed_to_seek_read_ptr_to_next_block",
        Some(format!(
            "Failed to seek read pointer to next block.\n {}",
            io_error
        )),
    )
}

// .... .... Storage::read_block .... ....

pub fn read_block_seek_block_offset(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Critical,
        "read_block_failed_to_seek_read_ptr_to_block_offset",
        Some(format!(
            "Could be logical issue: Failed to seek read pointer to block offset.\n {}",
            io_error
        )),
    )
}

pub fn read_block_seek_block_offset_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "read_block_failed_to_seek_read_ptr_to_block_offset",
        Some(format!(
            "Failed to seek read pointer to block offset.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}

pub fn read_block_read_block_header(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Happens,
        "read_block_failed_to_read_block_header",
        Some(format!(
            "Failed to read block header from file, check permissions and disk state.\n {}",
            io_error
        )),
    )
}

pub fn read_block_read_block_header_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "read_block_failed_to_read_block_header",
        Some(format!(
            "Failed to read block header from file, check permissions and path.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}

pub fn read_block_read_block_data(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Critical,
        "read_block_failed_to_read_block_data",
        Some(format!(
            "Possible logical error or storage file is corrupt: Failed to read block data, check permissions and disk state.\n {}",
            io_error
        )),
    )
}

pub fn read_block_read_block_data_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "read_block_failed_to_read_block_data",
        Some(format!(
            "Failed to read block data from file, check permissions and path.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}

// .... .... Storage::write_block .... ....

pub fn write_block_seek_block_offset(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Critical,
        "write_block_failed_to_seek_write_ptr_to_block_offset",
        Some(format!(
            "Could be logical issue: Failed to seek write pointer to block offset.\n {}",
            io_error
        )),
    )
}

pub fn write_block_seek_block_offset_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "write_block_failed_to_seek_write_ptr_to_block_offset",
        Some(format!(
            "Failed to seek write pointer to block offset.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}

pub fn write_block_write_block_header(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Happens,
        "write_block_failed_to_write_block_header",
        Some(format!(
            "Failed to write block header to file, check permissions and disk state.\n {}",
            io_error
        )),
    )
}

pub fn write_block_write_block_header_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "write_block_failed_to_write_block_header",
        Some(format!(
            "Failed to write block header to file, check permissions and path.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}

pub fn write_block_write_block_data(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Critical,
        "write_block_failed_to_write_block_data",
        Some(format!(
            "Possible logical issue or storage file is corrupt: Failed to write block data, check permissions and disk state.\n {}",
            io_error
        )),
    )
}

pub fn write_block_write_block_data_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "write_block_failed_to_write_block_data",
        Some(format!(
            "Failed to write block data to file, check permissions and path.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}

// .... .... Storage::delete_block .... ....

pub fn delete_block_seek_block_offset(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Critical,
        "delete_block_failed_to_seek_write_ptr_to_block_offset",
        Some(format!(
            "Could be logical issue: Failed to seek write pointer to block offset.\n {}",
            io_error
        )),
    )
}

pub fn delete_block_seek_block_offset_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "delete_block_failed_to_seek_write_ptr_to_block_offset",
        Some(format!(
            "Failed to seek write pointer to block offset.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}

pub fn delete_block_write_block_header(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Happens,
        "delete_block_failed_to_write_block_header",
        Some(format!(
            "Failed to write block header to file, check permissions and disk state.\n {}",
            io_error
        )),
    )
}

pub fn delete_block_write_block_header_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "delete_block_failed_to_write_block_header",
        Some(format!(
            "Failed to write block header to file, check permissions and path.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}

pub fn delete_block_write_block_data(io_error: std::io::Error) -> Error {
    Error::new(
        ErrorType::Critical,
        "delete_block_failed_to_write_block_data",
        Some(format!(
            "Possible logical issue or storage file is corrupt: Failed to write block data, check permissions and disk state.\n {}",
            io_error
        )),
    )
}

pub fn delete_block_write_block_data_success(bytes_read: usize) -> Error {
    Error::new(
        ErrorType::Unexpected,
        "delete_block_failed_to_write_block_data",
        Some(format!(
            "Failed to write block data to file, check permissions and path.\n\tBytes Read: {} bytes",
            bytes_read
        )),
    )
}
