use util::error::Error;
mod storage_errors;

/// 4 bytes for index for a block
pub type BlockIndex = u32;
/// 4 bytes to store, blockLength, blockSize
type BlockLength = u32; // stored in file

//  ... ... ... ... ... ... ... ... Storage Header ... ... ... ... ... ... ... ... ... ..

/// Main Header for storage file
/// - Stores constant capacity of each block as 4 bytes unsied integer as little endian
struct StorageHeader {
    block_len: BlockLength,
}

const STORAGE_HEADER_SIZE: usize = std::mem::size_of::<StorageHeader>();

impl StorageHeader {
    fn new(block_len: BlockLength) -> Self {
        StorageHeader { block_len }
    }

    fn from_bytes(bytes: [u8; STORAGE_HEADER_SIZE]) -> StorageHeader {
        let block_len = BlockLength::from_le_bytes(bytes);
        StorageHeader { block_len }
    }

    fn to_bytes(&self) -> [u8; STORAGE_HEADER_SIZE] {
        BlockLength::to_le_bytes(self.block_len)
    }
}

#[cfg(test)]
mod unit_tests_storage_header {
    use super::*;
    #[test]
    fn test_storage_header_to_bytes() {
        let storage_header = StorageHeader::new(16777472);
        let bytes = storage_header.to_bytes();
        assert_eq!(bytes, [0, 1, 0, 1]);
    }

    #[test]
    fn test_storage_header_from_bytes() {
        let storage_header = StorageHeader::from_bytes([0, 2, 0, 2]);
        assert_eq!(storage_header.block_len, 33554944);
    }

    #[test]
    fn test_storage_header_full_flow() {
        let block_length = 16777472;
        let expected_bytes = [0, 1, 0, 1];
        let storage_header = StorageHeader::new(block_length);
        assert_eq!(storage_header.block_len, block_length);
        let bytes = storage_header.to_bytes();
        assert_eq!(bytes, expected_bytes);
        let storage_header = StorageHeader::from_bytes(bytes);
        assert_eq!(storage_header.block_len, block_length);
    }
}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

//  ... ... ... ... ... ... ... ... Block Header ... ... ... ... ... ... ... ... ... ....

/// Header of each block
/// - Stores size of data stored in the block as 4 bytes unsied integer as little endian
struct BlockHeader {
    block_data_size: BlockLength,
}

pub const BLOCK_HEADER_SIZE: usize = std::mem::size_of::<BlockHeader>();

impl BlockHeader {
    fn new(block_data_size: u32) -> BlockHeader {
        BlockHeader { block_data_size }
    }

    fn from_bytes(bytes: [u8; BLOCK_HEADER_SIZE]) -> BlockHeader {
        let block_data_size = BlockLength::from_le_bytes(bytes);
        BlockHeader { block_data_size }
    }

    fn to_bytes(&self) -> [u8; BLOCK_HEADER_SIZE] {
        BlockLength::to_le_bytes(self.block_data_size)
    }
}

#[cfg(test)]
mod unit_test_block_header {
    use super::*;

    #[test]
    fn test_block_header_to_bytes() {
        let block_header = BlockHeader::new(16777472);
        let bytes = block_header.to_bytes();
        assert_eq!(bytes, [0, 1, 0, 1]);
    }

    #[test]
    fn test_block_header_from_bytes() {
        let block_header = BlockHeader::from_bytes([0, 2, 0, 2]);
        assert_eq!(block_header.block_data_size, 33554944);
    }

    #[test]
    fn test_block_header_full_flow() {
        let block_data_size = 16777472;
        let expected_bytes = [0, 1, 0, 1];
        let block_header = BlockHeader::new(block_data_size);
        assert_eq!(block_header.block_data_size, block_data_size);
        let bytes = block_header.to_bytes();
        assert_eq!(bytes, expected_bytes);
        let block_header = BlockHeader::from_bytes(bytes);
        assert_eq!(block_header.block_data_size, block_data_size);
    }
}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

// ... ... ... ... ... ... ... ... ... Storage ... ... ... ... ... ... ... ... ... ....

use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};

pub struct Storage {
    header: StorageHeader,
    /// Map of empty blocks in the storage file
    free_blocks: BTreeSet<BlockIndex>,
    /// Number of blocks in the storage file (used or free)
    end_block_count: BlockIndex,
    /// File object for writing
    file_writer: File,
    /// Index of last written byte in the file
    write_pointer: usize,
    /// File object for reading
    file_reader: File,
    /// Index of last read byte in the file
    read_pointer: usize,
}

impl Storage {
    pub fn block_len(&self) -> BlockLength {
        self.header.block_len
    }
    //  ... ... ... ... ... ... Static Functions ... ... ... ... ... ... .

    /// Open storage file for writing
    /// - creates a new file if it does not exist
    /// - truncate: if true, truncates the file to 0 bytes
    /// - truncate: if false, no modification to the file
    /// - returns: (file_object_for_writing, write_pointer) - write_pointer is always 0
    fn open_file_writer(file_path: &str, truncate: bool) -> Result<(File, usize), Error> {
        let file_path_clone = file_path.clone();
        let file_writer_result = OpenOptions::new()
            .write(true)
            .truncate(truncate)
            .create(true)
            .open(file_path_clone);
        if let Err(result_error) = file_writer_result {
            return Err(storage_errors::open_file_writer_open_file(result_error));
        }
        let file_writer = file_writer_result.unwrap();
        let write_pointer = 0;
        Ok((file_writer, write_pointer))
    }

    /// Open storage file for reading
    /// - returns: (file_object_for_reading, read_pointer) - read_pointer is always 0
    fn open_file_reader(file_path: &str) -> Result<(File, usize), Error> {
        let file_path_clone = file_path.clone();
        let file_reader_result = OpenOptions::new().read(true).open(file_path_clone);
        if let Err(result_error) = file_reader_result {
            return Err(storage_errors::open_file_reader_open_file(result_error));
        }
        let file_reader = file_reader_result.unwrap();
        let read_pointer = 0;
        Ok((file_reader, read_pointer))
    }

    // // ... ... ... ... ... Storage Constructors ... ... ... ... ... .

    /// Create new storage file
    /// - Create/Overwrite new storage file in given path
    /// - Initializes storage header
    pub fn new(file_path: String, block_len: u32) -> Result<Storage, Error> {
        let (file_writer, write_pointer) = Storage::open_file_writer(&file_path, true)?;
        let (file_reader, read_pointer) = Storage::open_file_reader(&file_path)?;

        // Initialize storage object
        let mut storage = Storage {
            header: StorageHeader::new(block_len),
            free_blocks: BTreeSet::new(),
            end_block_count: 0,
            file_writer,
            write_pointer,
            file_reader,
            read_pointer,
        };

        // Write storage header to file
        storage.set_storage_header()?;

        Ok(storage)
    }

    /// Open existing storage file
    /// - Loads storage header
    /// - Loads free blocks Set
    pub fn open(file_path: String) -> Result<Storage, Error> {
        let (file_writer, write_pointer) = Storage::open_file_writer(&file_path, false)?;
        let (file_reader, read_pointer) = Storage::open_file_reader(&file_path)?;

        // Initialize storage object
        let mut storage = Storage {
            header: StorageHeader::new(0),
            free_blocks: BTreeSet::new(),
            end_block_count: 0,
            file_writer,
            write_pointer,
            file_reader,
            read_pointer,
        };

        // - read and update storage header from file
        storage.get_storage_header()?;

        // - read file and count
        // -- total blocks - update self.end_block_count
        // -- free blocks - update self.free_blocks
        storage.read_storage_block_headers()?;

        Ok(storage)
    }
    // // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ....

    // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...

    // ... ... ... ... ... . InMemory Logic Functions ... ... ... ... ....

    /// check if block is within storage file, without reading it from file (in memory)
    fn block_exists(&mut self, block_index: BlockIndex) -> bool {
        block_index < self.end_block_count
    }

    /// Check if block is empty, without reading it from file (in memory)
    fn is_empty_block(&mut self, block_index: BlockIndex) -> bool {
        if self.block_exists(block_index) {
            self.free_blocks.contains(&block_index)
        } else {
            true
        }
    }

    // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...

    // ... ... ... ... ... ... File IO Functions ... ... ... ... ... ... .

    /// Set storage header in storage file
    /// - Write storage header to file
    /// - NOTE: This can only be used once when creating a new storage file
    /// - returns: write pointer
    fn set_storage_header(&mut self) -> Result<usize, Error> {
        use std::io::prelude::*;
        let file = &mut self.file_writer;
        // Write storage header to file
        let header_bytes = self.header.to_bytes();
        // -- seek writer pointer to beginning of file
        let ptr_seek_result = file.seek(std::io::SeekFrom::Start(0));
        if ptr_seek_result.is_err() {
            return Err(storage_errors::set_storage_header_seek_start(
                ptr_seek_result.unwrap_err(),
            ));
        }
        // -- write storage header
        self.write_pointer = ptr_seek_result.unwrap() as usize;
        let write_result = file.write(&header_bytes);
        if write_result.is_err() {
            return Err(storage_errors::set_storage_header_write_header(
                write_result.unwrap_err(),
            ));
        }
        // -- verify write operation was successful
        let write_size = write_result.unwrap();
        if write_size != STORAGE_HEADER_SIZE {
            return Err(storage_errors::set_storage_header_write_header_success(
                write_size,
            ));
        }
        self.write_pointer += write_size;
        Ok(self.write_pointer)
    }

    /// Get storage header from storage file
    /// - Read storage header from file
    /// - update storage header in object
    /// - returns: read pointer
    fn get_storage_header(&mut self) -> Result<usize, Error> {
        use std::io::prelude::*;
        let file = &mut self.file_reader;

        // - Read storage header from file
        // -- seek reader pointer to beginning of file
        let ptr_seek_result = file.seek(std::io::SeekFrom::Start(0));
        if let Err(result_error) = ptr_seek_result {
            return Err(storage_errors::get_storage_header_seek_start(result_error));
        }
        // -- read storage header
        let mut header_bytes = [0u8; STORAGE_HEADER_SIZE];
        self.read_pointer = ptr_seek_result.unwrap() as usize;
        let read_result = file.read(&mut header_bytes);
        if let Err(result_error) = read_result {
            return Err(storage_errors::get_storage_header_read_header(result_error));
        }
        // -- verify read operation was successful
        let read_size = read_result.unwrap();
        if read_size != STORAGE_HEADER_SIZE as usize {
            return Err(storage_errors::get_storage_header_read_header_success(
                read_size,
            ));
        }
        // -- update read pointer
        self.read_pointer += read_size;

        // - parse storage header
        let storage_header = StorageHeader::from_bytes(header_bytes);

        // - copy storage header to storage object
        self.header = storage_header;

        // - return read pointer
        Ok(read_size)
    }

    /// Count number of blocks in storage file
    /// - update self.end_block_count
    /// - update self.free_blocks
    /// - returns: read pointer
    fn read_storage_block_headers(&mut self) -> Result<usize, Error> {
        use std::io::prelude::*;
        let file = &mut self.file_reader;

        // - read file and count
        // -- total blocks - update self.end_block_count
        // -- free blocks - update self.free_blocks
        let mut free_blocks = BTreeSet::new();
        // -- seek reader pointer to end of STORAGE_HEADER_SIZE - offset of first block
        let ptr_seek_result = file.seek(std::io::SeekFrom::Start(STORAGE_HEADER_SIZE as u64));
        if let Err(result_error) = ptr_seek_result {
            return Err(
                storage_errors::read_storage_block_headers_seek_1st_block_offset(result_error),
            );
        }
        // -- traverse all blocks in file, untill end of file
        let mut block_index = 0;
        loop {
            // - read block header
            let mut block_header_bytes = [0u8; BLOCK_HEADER_SIZE];
            let read_result = file.read(&mut block_header_bytes);
            if let Err(result_error) = read_result {
                return Err(
                    storage_errors::read_storage_block_headers_read_block_header(result_error),
                );
            }
            // -- check end of file
            // -- verify read operation was successful
            let read_size = read_result.unwrap();
            if read_size == 0 {
                // end of file reached
                break;
            }
            if read_size != BLOCK_HEADER_SIZE {
                return Err(
                    storage_errors::read_storage_block_headers_read_block_header_success(read_size),
                );
            }
            // -- update read pointer
            self.read_pointer += read_size;
            // -- parse block header
            let block_header = BlockHeader::from_bytes(block_header_bytes);

            // - check if block is free
            if block_header.block_data_size == 0 {
                // -- add block to free blocks
                free_blocks.insert(block_index);
            }
            // -- increment block index
            block_index += 1;

            // - seek reader pointer to end of block
            let ptr_seek_result =
                file.seek(std::io::SeekFrom::Current(self.header.block_len as i64));
            if ptr_seek_result.is_err() {
                return Err(storage_errors::read_storage_block_headers_seek_next_block(
                    ptr_seek_result.unwrap_err(),
                ));
            }
            let ptr_seek_result = ptr_seek_result.unwrap() as usize;
            self.read_pointer = ptr_seek_result;
            // -- verify seek operation was successful
            if ptr_seek_result != self.read_pointer {
                // end of file reached
                break;
            }
        }

        // - update end block count
        self.end_block_count = block_index;

        // - update free blocks
        self.free_blocks = free_blocks;

        // - return
        Ok(self.read_pointer)
    }

    /// Read block data from storage file
    /// - return (read_pointer, block_data)
    pub fn read_block(&mut self, block_index: BlockIndex) -> Result<(usize, Vec<u8>), Error> {
        if self.is_empty_block(block_index) {
            // return current read_pointer and empty vector
            return Ok((self.read_pointer as usize, Vec::new()));
        }
        use std::io::prelude::*;
        let block_length = self.header.block_len;
        let block_offset: usize = STORAGE_HEADER_SIZE
            + block_index as usize * (BLOCK_HEADER_SIZE + block_length as usize);

        // - seek reader to block offset
        let seek_result = self
            .file_reader
            .seek(std::io::SeekFrom::Start(block_offset as u64));
        if let Err(result_error) = seek_result {
            return Err(storage_errors::read_block_seek_block_offset(result_error));
        }
        // -- verify seek operation was successful
        let seek_position = seek_result.unwrap() as usize;
        if seek_position != block_offset {
            return Err(storage_errors::read_block_seek_block_offset_success(
                seek_position,
            ));
        }
        self.read_pointer = seek_position;

        // - read block data length from inital 4 bytes
        let block_data_size_bytes = &mut [0u8; 4];
        let read_result = self.file_reader.read(block_data_size_bytes);
        if let Err(result_error) = read_result {
            return Err(storage_errors::read_block_read_block_header(result_error));
        }
        // -- verify read operation was successful
        let read_size = read_result.unwrap();
        if read_size != BLOCK_HEADER_SIZE {
            return Err(storage_errors::read_block_read_block_header_success(
                read_size,
            ));
        }
        self.read_pointer += read_size;
        let block_header = BlockHeader::from_bytes(*block_data_size_bytes);

        // - read block data to vec
        let mut block_data = vec![0u8; block_header.block_data_size as usize];
        let read_result = self.file_reader.read(&mut block_data[..]);
        if read_result.is_err() {
            return Err(storage_errors::read_block_read_block_data(
                read_result.unwrap_err(),
            ));
        }
        let read_size = read_result.unwrap();
        self.read_pointer += read_size;

        // - verify read operation was successful
        if read_size != block_header.block_data_size as usize {
            return Err(storage_errors::read_block_read_block_data_success(
                read_size,
            ));
        }

        // - return read_pointer and block_data
        Ok((self.read_pointer, block_data))
    }

    /// Write block data to storage file
    /// - return write_pointer
    pub fn write_block(&mut self, block_index: BlockIndex, data: &[u8]) -> Result<usize, Error> {
        use std::io::prelude::*;
        let block_length = self.header.block_len;
        let block_offset = STORAGE_HEADER_SIZE
            + block_index as usize * (BLOCK_HEADER_SIZE + block_length as usize);

        // - seek writer to block offset
        let seek_result = self
            .file_writer
            .seek(std::io::SeekFrom::Start(block_offset as u64));
        if seek_result.is_err() {
            return Err(storage_errors::write_block_seek_block_offset(
                seek_result.unwrap_err(),
            ));
        }
        // -- verify seek operation was successful
        let seek_position = seek_result.unwrap() as usize;
        if seek_position != block_offset {
            return Err(storage_errors::write_block_seek_block_offset_success(
                seek_position,
            ));
        }
        self.write_pointer = seek_position;

        // - Write Block Header
        // -- write block header to inital BLOCK_HEADER_SIZE bytes
        let block_header = BlockHeader::new(data.len() as BlockLength);
        let write_result = self.file_writer.write(&block_header.to_bytes());
        if write_result.is_err() {
            return Err(storage_errors::write_block_write_block_header(
                write_result.unwrap_err(),
            ));
        }
        let write_size = write_result.unwrap();
        self.write_pointer += write_size;
        // -- verify write operation was successful
        if write_size != BLOCK_HEADER_SIZE {
            return Err(storage_errors::write_block_write_block_header_success(
                write_size,
            ));
        }

        // - Write Block Data
        // -- write block data to file
        let write_result = self.file_writer.write(data);
        if write_result.is_err() {
            return Err(storage_errors::write_block_write_block_data(
                write_result.unwrap_err(),
            ));
        }
        let write_size = write_result.unwrap();
        self.write_pointer += write_size;
        // -- verify write operation was successful
        if write_size != data.len() {
            return Err(storage_errors::write_block_write_block_data_success(
                write_size,
            ));
        }

        // - update free_blocks map
        self.free_blocks.remove(&block_index);

        // - update max_block_index
        if block_index >= self.end_block_count {
            self.end_block_count = block_index + 1;
        }

        // - return write pointer
        Ok(self.write_pointer)
    }

    /// Write block data to storage file
    /// - return write_pointer
    pub fn delete_block(
        &mut self,
        block_index: BlockIndex,
        hard_delete: bool,
    ) -> Result<usize, Error> {
        if !self.block_exists(block_index)
            || (!hard_delete && self.free_blocks.contains(&block_index))
        {
            return Ok(self.write_pointer);
        }
        use std::io::prelude::*;
        let block_length = self.header.block_len;
        let block_offset = STORAGE_HEADER_SIZE
            + block_index as usize * (BLOCK_HEADER_SIZE + block_length as usize);

        // - seek writer to block offset
        let seek_result = self
            .file_writer
            .seek(std::io::SeekFrom::Start(block_offset as u64));
        if let Err(result_error) = seek_result {
            return Err(storage_errors::delete_block_seek_block_offset(result_error));
        }
        // -- verify seek operation was successful
        let seek_position = seek_result.unwrap() as usize;
        if seek_position != block_offset {
            return Err(storage_errors::delete_block_seek_block_offset_success(
                seek_position,
            ));
        }
        self.write_pointer = block_offset;

        // - Write Block Header
        // -- write block header to inital BLOCK_HEADER_SIZE bytes
        let block_header = BlockHeader::new(0);
        let write_result = self.file_writer.write(&block_header.to_bytes());
        if let Err(result_error) = write_result {
            return Err(storage_errors::delete_block_write_block_header(
                result_error,
            ));
        }
        let write_size = write_result.unwrap();
        self.write_pointer += write_size;
        // -- verify write operation was successful
        if write_size != BLOCK_HEADER_SIZE {
            return Err(storage_errors::delete_block_write_block_header_success(
                write_size,
            ));
        }

        // - hard delete block
        if hard_delete {
            // post successful block header write, writer pointer must be at data offset
            // - overwrite full block with zeros
            let block_data_of_zeros = vec![0u8; block_length as usize];
            let write_result = self.file_writer.write(&block_data_of_zeros[..]);
            if let Err(result_error) = write_result {
                return Err(storage_errors::delete_block_write_block_data(result_error));
            }
            let write_size = write_result.unwrap();
            // -- verify write operation was successful
            if write_size != block_length as usize {
                return Err(storage_errors::delete_block_write_block_data_success(
                    write_size,
                ));
            }
            // -- increment write pointer
            self.write_pointer += write_size;
        }

        // update free_blocks map
        self.free_blocks.insert(block_index);

        // return write pointer
        Ok(self.write_pointer)
    }

    // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...

    // ... ... ... ... ... ... Abstract Functions ... ... ... ... ... ... .

    /// Return blocks in storage to write data to, in assending order of index
    /// - collect free blocks
    /// - if free blocks not enough, extend storage
    pub fn search_block_allocation_indexes(&self, count: BlockIndex) -> Vec<BlockIndex> {
        let mut available_free_blocks = self
            .free_blocks
            .iter()
            .cloned()
            .collect::<Vec<BlockIndex>>();
        available_free_blocks.truncate(count as usize);
        available_free_blocks.sort_unstable();
        // push indexes beyond end_block_count if required
        for i in (self.end_block_count as usize)
            ..(self.end_block_count as usize + count as usize - available_free_blocks.len())
        {
            available_free_blocks.push(i as BlockIndex);
        }
        available_free_blocks
    }

    // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..
