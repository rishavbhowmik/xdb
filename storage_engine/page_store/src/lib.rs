pub mod page_usize;

use util::error::{Error, ErrorType};
use util::{Env, ENV};

use page_usize::{page_usize_from_le_bytes, page_usize_to_le_bytes, PageUsizeType};

pub struct PageStoreHeaders {
    page_len: u64,
}
impl PageStoreHeaders {
    pub fn new(page_len: usize) -> Self {
        PageStoreHeaders {
            page_len: page_len as u64,
        }
    }
    pub fn to_bytes(&self) -> [u8; 8] {
        u64::to_le_bytes(self.page_len)
    }
    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        let page_len = u64::from_le_bytes(bytes);
        PageStoreHeaders { page_len }
    }
}

const PAGE_HEADER_LEN: usize = std::mem::size_of::<PageStoreHeaders>();

pub struct PageSettings {
    page_size_type: PageUsizeType,
    page_capacity: usize,
    page_len: usize,
}
impl PageSettings {
    pub fn new(page_len: usize) -> Self {
        let page_size_type = PageUsizeType::for_max_usize(page_len);
        let page_capacity = page_len - PageUsizeType::size_of(page_size_type);
        PageSettings {
            page_size_type,
            page_capacity,
            page_len,
        }
    }
    pub fn page_capacity(&self) -> usize {
        self.page_capacity
    }
}

pub struct PageStore {
    pub page_settings: PageSettings,
    /// last_page_index + 1, which means total number of pages in the store (allocated and empty)
    page_count: usize,
    /// Set of page_index of empty pages (allocated before but deleted later)
    empty_page_index_set: std::collections::BTreeSet<usize>,
    /// file ptr for storage file
    file: std::fs::File,
}

impl PageStore {
    /// Read page store header from file and return parsed page store header
    fn read_page_store_header(file: &mut std::fs::File) -> Result<PageStoreHeaders, Error> {
        let mut buffer = [0u8; PAGE_HEADER_LEN];

        // Seek to the beginning of the file & read page header
        use std::io::{Read, Seek};
        match file.seek(std::io::SeekFrom::Start(0)) {
            Ok(_) => (),
            Err(e) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "read_page_store_header_seek_failed",
                    Some(format!("{}", e)),
                ))
            }
        }
        match file.read_exact(&mut buffer) {
            Ok(_) => (),
            Err(e) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "read_page_store_header_read_failed",
                    Some(format!("{}", e)),
                ));
            }
        }

        // Parse the page header
        Ok(PageStoreHeaders::from_bytes(buffer))
    }

    /// Write page store header to file
    /// The page header is the first 8 bytes of the file
    fn write_page_store_header(
        &mut self,
        page_store_headers: &PageStoreHeaders,
    ) -> Result<(), Error> {
        let bytes = page_store_headers.to_bytes();

        // Seek to the beginning of the file & write page header
        use std::io::{Seek, Write};
        match self.file.seek(std::io::SeekFrom::Start(0)) {
            Ok(_) => (),
            Err(e) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "write_page_store_header_seek_failed",
                    Some(format!("{}", e)),
                ))
            }
        }
        match self.file.write(&bytes) {
            Ok(bytes_written) => {
                if bytes_written != PAGE_HEADER_LEN {
                    return Err(Error::new(
                        ErrorType::Critical,
                        "write_page_store_header_write_failed",
                        Some(format!(
                            "write {} bytes, expected {} bytes",
                            bytes_written, PAGE_HEADER_LEN
                        )),
                    ));
                }
            }
            Err(e) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "write_page_store_header_write_failed",
                    Some(format!("{}", e)),
                ))
            }
        }

        Ok(())
    }

    /// Page indexes to write data to
    /// - Collect empty page indexes
    /// - If enough empty_pages are not found, then append new pages beyond the last page
    /// - Returns page_indexes in assending order
    pub fn get_page_indexes_for_writes(&self, page_count: usize) -> Result<Vec<usize>, Error> {
        let mut page_indexes: Vec<usize> = vec![];

        // Collect empty page indexes (ascending order)
        // BtreeSet::iter() iterates in ascending order of items in the BTree
        for page_index in self.empty_page_index_set.iter().cloned() {
            if page_indexes.len() >= page_count {
                break;
            }
            page_indexes.push(page_index);
        }

        // If not enough empty pages, append new pages beyond the last page
        if page_indexes.len() < page_count {
            let extended_page_index_range =
                (self.page_count)..(self.page_count + page_count - page_indexes.len());
            page_indexes.extend(extended_page_index_range);
        }

        Ok(page_indexes) // Already in ascending order [...empty_page_indexes, ...page_indexes_beyond_last_page]
    }

    fn page_offset(&self, page_index: usize) -> usize {
        PAGE_HEADER_LEN + page_index * self.page_settings.page_len
    }

    /// Write data to page store file at page index
    /// ### Parameters
    /// - `page_index`: page index to write data to
    /// - `page_payload`: data to write to page store file
    /// - `force_payload_size_0`: if true, store payload size as 0. (Used for hard delete, where we need to set payload size to 0 but also fill the page with something.)
    /// ### Order
    /// - Write operations must be performed in assending order of page indexes
    pub fn write_page(
        &mut self,
        page_index: usize,
        page_payload: &[u8],
        force_payload_size_0: bool,
    ) -> Result<(), Error> {
        if matches!(ENV, Env::Dev | Env::Test) {
            // Check if page_data is too large
            if page_payload.len() > self.page_settings.page_capacity {
                return Err(Error::new(
                    ErrorType::Unexpected,
                    "page_store_write_page_payload_too_large",
                    Some(format!("page_index: {}", page_index)),
                ));
            }

            // Ignore if trying to delete the page
            // & Check if page index is already allocated
            if !(page_payload.is_empty() || force_payload_size_0)
                && page_index < self.page_count
                && !self.empty_page_index_set.contains(&page_index)
            {
                return Err(Error::new(
                    ErrorType::Unexpected,
                    "page_store_write_page_not_empty_page",
                    Some(format!("page_index: {}", page_index)),
                ));
            }

            // Check if page index is not far beyond the last page
            // `page_index` must be in [0 ..= page_count].
            if page_index > self.page_count {
                return Err(Error::new(
                    ErrorType::Warning,
                    "page_store_write_page_index_out_of_range",
                    Some(format!(
                        "page_index: {}, page_count: {}",
                        page_index, self.page_count
                    )),
                ));
            }
        }

        // Serialize page buffer
        let page_buffer = [
            // Payload size as bytes - The page header
            page_usize_to_le_bytes(
                if force_payload_size_0 {
                    0
                } else {
                    page_payload.len()
                },
                self.page_settings.page_size_type,
            ),
            // Payload bytes - The page payload
            page_payload.to_vec(),
        ]
        .concat();

        // Seek to page offset & write page payload
        let page_offset = self.page_offset(page_index);
        use std::io::{Seek, Write};
        match self.file.seek(std::io::SeekFrom::Start(page_offset as u64)) {
            Ok(_) => (),
            Err(err) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "page_store_write_page_seek_fail",
                    Some(format!("page_index: {}, err: {:?}", page_index, err)),
                ))
            }
        }
        match self.file.write(&page_buffer) {
            Ok(bytes_written) => {
                if bytes_written != page_buffer.len() {
                    return Err(Error::new(
                        ErrorType::Critical,
                        "page_store_write_page_write_fail",
                        Some(format!(
                            "Failed to write all bytes: page_index: {}",
                            page_index
                        )),
                    ));
                }
            }
            Err(err) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "page_store_write_page_write_fail",
                    Some(format!(
                        "Error on writing: page_index: {}, err: {:?}",
                        page_index, err
                    )),
                ))
            }
        }

        // Remove page index from empty page index set
        self.empty_page_index_set.remove(&page_index);

        // If page_index point to new last page, increase page count
        // NOTE: Case where page index is greater than page_count is handled in debugging mode
        if page_index == self.page_count {
            self.page_count += 1;
        }

        Ok(())
    }

    /// Read data from page store file at page index
    /// ### Parameter
    /// - `page_index`: page index to read data from
    /// - `read_start`: Option<usize>,
    /// - `read_end`: Option<usize>
    /// ### Order
    /// - Read operations must be performed in assending order of page indexes
    pub fn read_page(
        &mut self,
        page_index: usize,
        read_start: Option<usize>,
        read_end: Option<usize>,
    ) -> Result<Vec<u8>, Error> {
        if matches!(ENV, Env::Dev | Env::Test) {
            // Check if page index is not far beyond the last page
            // `page_index` must be in [0 .. page_count].
            if page_index >= self.page_count {
                return Err(Error::new(
                    ErrorType::Warning,
                    "page_store_read_page_index_out_of_range",
                    Some(format!(
                        "page_index: {}, page_count: {}",
                        page_index, self.page_count
                    )),
                ));
            }
        }

        // CanDo: possible optimization: Return vec![] if page_index is present in empty_page_index_set

        // Seek to page offset
        let page_offset = self.page_offset(page_index);
        use std::io::{Read, Seek};
        match self.file.seek(std::io::SeekFrom::Start(page_offset as u64)) {
            Ok(_) => (),
            Err(err) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "page_store_read_page_seek_fail",
                    Some(format!("page_index: {}, err: {:?}", page_index, err)),
                ))
            }
        }

        // Read & parse page_payload_size
        let page_payload_size_buf_len = PageUsizeType::size_of(self.page_settings.page_size_type); // Size of 'int as bytes'
        let mut page_payload_size_bytes = vec![0; page_payload_size_buf_len];
        match self.file.read_exact(&mut page_payload_size_bytes) {
            Ok(_) => (),
            Err(e) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "read_page_store_header_read_failed",
                    Some(format!("{}", e)),
                ));
            }
        }
        let page_payload_size =
            page_usize_from_le_bytes(&page_payload_size_bytes, self.page_settings.page_size_type);

        // Compute start & end point of page_body to read
        let page_body_read_start = match read_start {
            Some(start) => start,
            None => 0,
        };
        let page_body_read_end = match read_end {
            Some(end) => {
                if end > page_payload_size {
                    page_payload_size
                } else {
                    end
                }
            }
            None => page_payload_size,
        };

        if page_body_read_end <= page_body_read_start {
            return Ok(vec![]);
        }

        let page_body_read_len = page_body_read_end - page_body_read_start;

        // Seek to page body start offset & read page_payload of size page_body_read_len bytes
        if page_body_read_start > 0 {
            match self
                .file
                .seek(std::io::SeekFrom::Current(page_body_read_start as i64))
            {
                Ok(_) => (),
                Err(err) => {
                    return Err(Error::new(
                        ErrorType::Critical,
                        "page_store_read_page_seek_fail",
                        Some(format!(
                            "page_index: {}, page_body_start_offset: {}, err: {:?}",
                            page_index, page_body_read_start, err
                        )),
                    ))
                }
            }
        }
        let mut page_payload_bytes = vec![0; page_body_read_len];
        match self.file.read_exact(&mut page_payload_bytes) {
            Ok(_) => (),
            Err(e) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "read_page_store_header_read_failed",
                    Some(format!("{}", e)),
                ));
            }
        }

        Ok(page_payload_bytes)
    }

    /// Read size of page payload at page index
    /// ### Parameter
    /// - `page_index`: page index to read data from
    /// ### Order
    /// - Read operations must be performed in assending order of page indexes
    pub fn read_page_payload_size(&mut self, page_index: usize) -> Result<usize, Error> {
        if matches!(ENV, Env::Dev | Env::Test) {
            // Check if page index is not far beyond the last page
            // `page_index` must be in [0 .. page_count].
            if page_index >= self.page_count {
                return Err(Error::new(
                    ErrorType::Warning,
                    "page_store_read_page_payload_size_index_out_of_range",
                    Some(format!(
                        "page_index: {}, page_count: {}",
                        page_index, self.page_count
                    )),
                ));
            }
        }

        // CanDo: possible optimization: Return 0 if page_index is present in empty_page_index_set

        // Seek to page offset
        let page_offset = self.page_offset(page_index);
        use std::io::{Read, Seek};
        match self.file.seek(std::io::SeekFrom::Start(page_offset as u64)) {
            Ok(_) => (),
            Err(err) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "page_store_read_page_payload_size_seek_fail",
                    Some(format!("page_index: {}, err: {:?}", page_index, err)),
                ))
            }
        }

        // Read & parse page_payload_size
        let page_payload_size_buf_len = PageUsizeType::size_of(self.page_settings.page_size_type);
        let mut page_payload_size_bytes = vec![0; page_payload_size_buf_len];
        match self.file.read_exact(&mut page_payload_size_bytes) {
            Ok(_) => (),
            Err(e) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "read_page_store_header_read_failed",
                    Some(format!("{}", e)),
                ));
            }
        }

        Ok(page_usize_from_le_bytes(
            &page_payload_size_bytes,
            self.page_settings.page_size_type,
        ))
    }

    pub fn delete_page(&mut self, page_index: usize, hard_delete: bool) -> Result<(), Error> {
        if matches!(ENV, Env::Dev | Env::Test) {
            // Check if page index is not out of range
            // `page_index` must be in [0 .. page_count].
            if page_index > self.page_count {
                return Err(Error::new(
                    ErrorType::Warning,
                    "page_store_delete_page_page_index_out_of_range",
                    Some(format!(
                        "page_index: {}, page_count: {}",
                        page_index, self.page_count
                    )),
                ));
            }
        }

        // Remove page index from empty page index set
        match hard_delete {
            true => {
                // Overwrite page index with 0s
                self.write_page(page_index, &vec![0; self.page_settings.page_capacity], true)?
                // Note:- Overwriting only the allocated bytes after reading page_payload_size is not worth an extra disk cycle(required to page_payload_size=0 in the end).
            }
            false => self.write_page(page_index, &[], false)?,
        }

        // Insert page index into empty page index set
        self.empty_page_index_set.insert(page_index);

        Ok(())
        // Note: page_count is only incremental and depends last page index which is incremental too
    }

    pub fn write_many_pages(&mut self, page_payload_map: &[(usize, &[u8])]) -> Result<(), Error> {
        for (page_index, page_payload) in page_payload_map {
            self.write_page(*page_index, page_payload, false)?;
        }
        Ok(())
    }

    /// Safely write multiple pages
    pub fn auto_write_pages(&mut self, payloads: &[&[u8]]) -> Result<Vec<usize>, Error> {
        // allocate pages
        let page_index_list = self.get_page_indexes_for_writes(payloads.len())?;

        for (page_index, payload) in page_index_list.iter().zip(payloads.iter()) {
            match self.write_page(*page_index, payload, false) {
                Ok(_) => (),
                Err(err) => {
                    // Roll back attempt to write the pages
                    for page_index in page_index_list.iter().rev() {
                        self.delete_page(*page_index, false)?;
                    }
                    return Err(err);
                }
            }
        }

        Ok(page_index_list)
    }

    /// ### Constructor: Open new page store (truncates the file if exists)
    pub fn open_new(file_path: &str, page_len: usize) -> Result<PageStore, Error> {
        // Open new file
        let file = match std::fs::File::options()
            .create_new(true)
            .read(true)
            .write(true)
            .truncate(true)
            .append(false)
            .open(file_path)
        {
            Ok(file) => file,
            Err(e) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "page_store_open_new_file_open_failed",
                    Some(format!("{}", e)),
                ))
            }
        };

        // Initialize page store
        let page_settings = PageSettings::new(page_len);
        let page_count: usize = 0;
        let empty_page_index_set: std::collections::BTreeSet<usize> =
            std::collections::BTreeSet::new();
        let mut page_store = PageStore {
            page_settings,
            page_count,
            empty_page_index_set,
            file,
        };

        // Write page header to file
        let page_store_header = PageStoreHeaders::new(page_len);
        page_store.write_page_store_header(&page_store_header)?;

        Ok(page_store)
    }

    /// ### Constructor: Open existing page store
    pub fn open_existing(file_path: &str) -> Result<PageStore, Error> {
        // Open existing storage file
        let mut file = match std::fs::File::options()
            .create_new(false)
            .read(true)
            .write(true)
            .truncate(false)
            .append(false)
            .open(file_path)
        {
            Ok(file) => file,
            Err(e) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "page_store_open_existing_file_open_failed",
                    Some(format!("{}", e)),
                ))
            }
        };

        // Check file size
        let file_size = match file.metadata() {
            Ok(metadata) => metadata.len() as usize,
            Err(e) => {
                return Err(Error::new(
                    ErrorType::Critical,
                    "page_store_open_existing_file_metadata_failed",
                    Some(format!("{}", e)),
                ))
            }
        };

        // Read page header from file (includes page_length)
        let page_store_headers = Self::read_page_store_header(&mut file)?;

        let page_len = page_store_headers.page_len as usize;
        let page_settings = PageSettings::new(page_len);

        // Calculate page count
        let total_page_len_sum = file_size - PAGE_HEADER_LEN;
        let page_count = (total_page_len_sum / page_len)
            + if (total_page_len_sum % page_len) > 0 {
                1
            } else {
                0
            };

        // Initialize page store object
        let mut page_store = PageStore {
            page_settings,
            page_count,
            empty_page_index_set: std::collections::BTreeSet::new(),
            file,
        };

        // Populate empty_page_index_set of page store object
        for page_index in 0..page_count {
            let page_payload_size = page_store.read_page_payload_size(page_index)?;
            if page_payload_size == 0 {
                page_store.empty_page_index_set.insert(page_index);
            }
        }

        Ok(page_store)
    }
}

// implement assertion traits for PageStore
impl std::fmt::Debug for PageStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PageStore {{ page_count={}, page_size_type={}, page_len={}, page_capacity={}, empty_page_index_set={:?} }}",
            self.page_count,
            match self.page_settings.page_size_type {
                PageUsizeType::U8 => "u8",
                PageUsizeType::U16 => "u16",
                PageUsizeType::U32 => "u32",
                PageUsizeType::U64 => "u64",
            },
            self.page_settings.page_len,
            self.page_settings.page_capacity,
            self.empty_page_index_set,
        )
    }
}

#[cfg(test)]
mod page_settings_test {
    use super::*;
    #[test]
    pub fn test_page_settings_u8() {
        let page_len = 1 as usize;
        let page_settings = PageSettings::new(page_len);
        assert!(matches!(page_settings.page_size_type, PageUsizeType::U8));
        assert_eq!(page_settings.page_len, page_len);
        assert_eq!(page_settings.page_capacity(), page_len - 1);

        let page_len = u8::MAX as usize;
        let page_settings = PageSettings::new(page_len);
        assert!(matches!(page_settings.page_size_type, PageUsizeType::U8));
        assert_eq!(page_settings.page_len, page_len);
        assert_eq!(page_settings.page_capacity(), page_len - 1);
    }
    #[test]
    pub fn test_page_settings_u16() {
        let page_len = u8::MAX as usize + 1;
        let page_settings = PageSettings::new(page_len);
        assert!(matches!(page_settings.page_size_type, PageUsizeType::U16));
        assert_eq!(page_settings.page_len, page_len);
        assert_eq!(page_settings.page_capacity(), page_len - 2);

        let page_len = u16::MAX as usize;
        let page_settings = PageSettings::new(page_len);
        assert!(matches!(page_settings.page_size_type, PageUsizeType::U16));
        assert_eq!(page_settings.page_len, page_len);
        assert_eq!(page_settings.page_capacity(), page_len - 2);
    }
    #[test]
    pub fn test_page_settings_u32() {
        let page_len = u16::MAX as usize + 1;
        let page_settings = PageSettings::new(page_len);
        assert!(matches!(page_settings.page_size_type, PageUsizeType::U32));
        assert_eq!(page_settings.page_len, page_len);
        assert_eq!(page_settings.page_capacity(), page_len - 4);

        let page_len = u32::MAX as usize;
        let page_settings = PageSettings::new(page_len);
        assert!(matches!(page_settings.page_size_type, PageUsizeType::U32));
        assert_eq!(page_settings.page_len, page_len);
        assert_eq!(page_settings.page_capacity(), page_len - 4);
    }
    #[test]
    pub fn test_page_settings_u64() {
        let page_len = u32::MAX as usize + 1;
        let page_settings = PageSettings::new(page_len);
        assert!(matches!(page_settings.page_size_type, PageUsizeType::U64));
        assert_eq!(page_settings.page_len, page_len);
        assert_eq!(page_settings.page_capacity(), page_len - 8);

        let page_len = u64::MAX as usize;
        let page_settings = PageSettings::new(page_len);
        assert!(matches!(page_settings.page_size_type, PageUsizeType::U64));
        assert_eq!(page_settings.page_len, page_len);
        assert_eq!(page_settings.page_capacity(), page_len - 8);
    }
}
