use std::{convert::TryInto, mem, sync::Mutex};

use page_store::PageStore;
use util::{
    error::{Error, ErrorType},
    make_chunks,
};

const U32_BYTES: usize = mem::size_of::<u32>();

// struct LastPageCache {
//     page_index: usize,
//     data: Vec<u8>,
// }

// pub struct PageListController {
//     page_store_object: PageStore, // Needs to be Mutex
//     start_page_index: usize,
//     page_count: usize,
//     last_page_cache: LastPageCache,
// }

// fn parse_page(page_store: &mut PageStore, page_index: usize, parse_data: bool) -> Result<(usize, Option<Vec<u8>>), Error> {
//     if parse_data {
//         let payload = page_store.read_page(page_index, None, None)?;
//         let next_page_index_as_bytes: [u8; 4] = payload[..U32_BYTES].try_into().unwrap();
//         let next_page_index: usize = u32::from_le_bytes(next_page_index_as_bytes).try_into().unwrap();
//         let data = payload[U32_BYTES..].to_vec();
//         Ok((next_page_index, Some(data)))
//     } else {
//         let payload = page_store.read_page(page_index, None, None)?;
//         let next_page_index_as_bytes: [u8; 4] = payload[..U32_BYTES].try_into().unwrap();
//         let next_page_index = u32::from_le_bytes(next_page_index_as_bytes).try_into().unwrap();
//         Ok((next_page_index, None))
//     }
// }

// impl PageListController {
//     pub fn new(page_store: &mut PageStore, start_page_index: usize) -> Result<Self, Error> {
//         let mut page_list_controller = PageListController {
//             page_store_object: page_store,
//             start_page_index: start_page_index,
//             page_count: 0,
//             last_page_cache: LastPageCache { page_index: 0, data: vec![] }
//         };

//         // Travese to last page in list
//         let mut next_page_index = page_list_controller.start_page_index;
//         loop {
//             let page_store = page_list_controller.page_store_object;
//             parse_page(&mut page_store, next_page_index, true)?;
//         }

//         Ok(page_list_controller)
//     }

//     fn last_page_void_size(&self) -> usize {
//         self.page_store_object.page_settings.page_capacity()
//             - U32_BYTES
//             - self.last_page_cache.data.len()
//     }

//     fn update_last_page_cache(&mut self, page_index: usize, page_data: Vec<u8>) {
//         self.last_page_cache.page_index = page_index;
//         self.last_page_cache.data = page_data;
//     }

//     // - Returns (first_page_index, last_page_index, last_page_data)
//     fn append_new_pages(&mut self, data: &[u8]) -> Result<(usize, usize, Vec<u8>), Error> {
//         let (chunk_count, chunks) =
//             util::make_chunks(&data, self.page_store_object.page_settings.page_capacity());
//         let page_index_list = self
//             .page_store_object
//             .get_page_indexes_for_writes(chunk_count)?;
//         let page_index_payload_map: Vec<(usize, Vec<u8>)> = chunks
//             .clone()
//             .enumerate()
//             .map(|(i, data)| {
//                 let page_index = page_index_list[i];
//                 let next_page_index = if (i + 1) < page_index_list.len() {
//                     page_index_list[i + 1]
//                 } else {
//                     0 // last page
//                 };
//                 let page_payload = [
//                     u32::to_le_bytes(next_page_index.try_into().unwrap()).to_vec(),
//                     data.to_vec(),
//                 ]
//                 .concat();
//                 (page_index, page_payload)
//             })
//             .collect();

//         for (page_index, page_payload) in page_index_payload_map {
//             self.page_store_object
//                 .write_page(page_index, &page_payload, false)?;
//         }

//         self.page_count += page_index_list.len();

//         match (
//             page_index_list.first(),
//             page_index_list.last(),
//             chunks.last(),
//         ) {
//             (Some(first_page_index), Some(last_page_index), Some(last_page_data)) => Ok((
//                 first_page_index.clone(),
//                 last_page_index.clone(),
//                 last_page_data.to_vec(),
//             )),
//             _ => Err(Error::new(
//                 ErrorType::Unexpected,
//                 "append_new_pages_not_first_last_page_index",
//                 None,
//             )),
//         }
//     }

//     pub fn append(&mut self, data: &[u8]) -> Result<(), Error> {
//         match self.page_count {
//             0 => {
//                 let (_, new_last_page_index, new_last_page_data) = self.append_new_pages(data)?;
//                 self.update_last_page_cache(new_last_page_index, new_last_page_data);
//                 Ok(())
//             }
//             _ => {
//                 let last_page_void_size = self.last_page_void_size();
//                 let last_page_data = [
//                     self.last_page_cache.data.clone(),
//                     data[..last_page_void_size].to_vec(),
//                 ]
//                 .concat();

//                 let data_for_new_pages = &data[last_page_void_size..];
//                 if data_for_new_pages.len() > 0 {
//                     let (first_page_index, new_last_page_index, new_last_page_data) =
//                         self.append_new_pages(data_for_new_pages)?;

//                     let last_page_payload = [
//                         u32::to_le_bytes(first_page_index.try_into().unwrap()).to_vec(),
//                         last_page_data,
//                     ]
//                     .concat();

//                     self.page_store_object.write_page(
//                         self.last_page_cache.page_index,
//                         &last_page_payload,
//                         false,
//                     )?;

//                     self.update_last_page_cache(new_last_page_index, new_last_page_data);
//                     Ok(())
//                 } else {
//                     Ok(())
//                 }
//             }
//         }
//     }
// }

type PageIndex = u32;

const PAGE_INDEX_BYTE_LEN: usize = mem::size_of::<PageIndex>();

struct PageListAttributes {
    start_page_index: usize,
    tail_page_index: usize,
    tail_page_data_cache: Vec<u8>,
}

impl PageListAttributes {
    fn page_data_chunk_capacity(page_capacity: usize) -> usize {
        page_capacity - U32_BYTES
    }
    fn last_page_void_size(&self, page_capacity: usize) -> usize {
        page_capacity - U32_BYTES - self.tail_page_data_cache.len()
    }
    fn update_tail_page_cache(&mut self, index: usize, data: &[u8]) {
        self.tail_page_index = index;
        self.tail_page_data_cache = data.to_vec();
    }
}
impl std::fmt::Debug for PageListAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something")
    }
}

pub struct PageListController {
    page_store_mutex: Mutex<PageStore>,
    page_list_array_mutex: Vec<Mutex<PageListAttributes>>,
}

const PAGE_INDEX_NULL: usize = PageIndex::MAX as usize;

fn raw_next_page_index_to_option(raw_next_page_index: usize) -> Option<usize> {
    match raw_next_page_index {
        PAGE_INDEX_NULL => None,
        _ => Some(raw_next_page_index),
    }
}

impl PageListController {
    fn parse_page(
        page_store: &mut PageStore,
        page_index: usize,
        parse_data: bool,
    ) -> Result<(Option<usize>, Option<Vec<u8>>), Error> {
        if parse_data {
            let payload = page_store.read_page(page_index, None, None)?;
            let next_page_index_as_bytes: [u8; 4] = payload[..U32_BYTES].try_into().unwrap();
            let next_page_index: usize = u32::from_le_bytes(next_page_index_as_bytes)
                .try_into()
                .unwrap();
            let data = payload[U32_BYTES..].to_vec();
            Ok((raw_next_page_index_to_option(next_page_index), Some(data)))
        } else {
            let payload = page_store.read_page(page_index, None, None)?;
            let next_page_index_as_bytes: [u8; 4] = payload[..U32_BYTES].try_into().unwrap();
            let next_page_index = u32::from_le_bytes(next_page_index_as_bytes)
                .try_into()
                .unwrap();
            Ok((raw_next_page_index_to_option(next_page_index), None))
        }
    }
    /// Bug - page_store_read_page_index_out_of_range
    fn load_page_list_attributes(
        page_store: &mut PageStore,
        start_page_index: usize,
    ) -> Result<PageListAttributes, Error> {
        let mut page_index_ptr = start_page_index;
        loop {
            let (next_page_index_option, _) = Self::parse_page(page_store, page_index_ptr, false)?;
            match next_page_index_option {
                Some(next_page_index) => page_index_ptr = next_page_index,
                None => {
                    let (_, last_page_cache) = Self::parse_page(page_store, page_index_ptr, true)?;
                    return Ok(PageListAttributes {
                        start_page_index,
                        tail_page_index: page_index_ptr,
                        tail_page_data_cache: last_page_cache.unwrap(),
                    });
                }
            }
        }
    }
    pub fn new<'a>(
        mut page_store_mutex: Mutex<PageStore>,
        page_list_start_indexes: Vec<usize>,
        // file_path: &str,
        // page_len: usize,
    ) -> Result<Self, Error> {
        // let mut page_store = PageStore::open_new(file_path, page_len)?;
        let mut page_store = page_store_mutex.get_mut().unwrap();

        let page_list_array_mutex_result: Result<Vec<_>, Error> = page_list_start_indexes
            .iter()
            .map(
                |start_index: &usize| -> Result<Mutex<PageListAttributes>, Error> {
                    let page_list_attributes =
                        Self::load_page_list_attributes(&mut page_store, *start_index)?;
                    Ok(Mutex::new(page_list_attributes))
                },
            )
            .collect();

        if page_list_array_mutex_result.is_err() {
            println!("{:?}", page_list_array_mutex_result.unwrap_err());
            return Err(Error::new(
                ErrorType::Critical,
                "CHEJEJDFJFJ",
                None,
                // Some(page_list_array_mutex_result.unwrap_err().code().to_string()),
            ));
        }

        // let page_store_mutex = Mutex::new(page_store);

        return Ok(Self {
            page_store_mutex,
            page_list_array_mutex: page_list_array_mutex_result.unwrap(),
        });
    }
    fn create_list(&mut self, data: Vec<u8>) -> Result<(), Error> {
        let mut page_store = self.page_store_mutex.lock().unwrap();

        // Create data chunks
        let (new_page_count, new_page_data_chunks) = make_chunks(
            &data,
            PageListAttributes::page_data_chunk_capacity(page_store.page_settings.page_capacity()),
        );
        // Allocate page_index to write data
        let new_page_index_list = page_store.get_page_indexes_for_writes(new_page_count)?;

        // Add next_page_index to payload of eachpage - Move this to seprate function, as it's repeatative
        let page_index_payload_map = new_page_data_chunks
            .into_iter()
            .enumerate()
            .map(|(i, data)| {
                let page_index = new_page_index_list[i];
                let next_page_index = match new_page_count - i {
                    1 => u32::MAX as usize, // last page
                    _ => new_page_index_list[i + 1],
                };
                (page_index, next_page_index, data)
            })
            .into_iter();

        // Write data to new pages - Move this to seprate function, as it's repeatative
        for (page_index, next_page_index, data) in page_index_payload_map.clone() {
            let page_payload: Vec<u8> =
                [next_page_index.to_le_bytes().to_vec(), data.to_vec()].concat();
            page_store.write_page(page_index, &page_payload, false)?;
        }

        // Create page_list_attribute
        let (start_page_index, tail_page_index, tail_page_data_cache) =
            match (new_page_index_list.first(), page_index_payload_map.last()) {
                (Some(start_page_index), Some((tail_page_index, _, tail_page_data_cache))) => (
                    *start_page_index,
                    tail_page_index,
                    tail_page_data_cache.to_vec(),
                ),
                _ => unimplemented!(),
            };
        self.page_list_array_mutex
            .push(Mutex::new(PageListAttributes {
                start_page_index,
                tail_page_index,
                tail_page_data_cache,
            }));

        Ok(())
    }
    pub fn append_list(self, page_list_start_index: usize, data: Vec<u8>) -> Result<(), Error> {
        let mut page_store = self.page_store_mutex.lock().unwrap();

        let page_list_attribute_mutex = match self
            .page_list_array_mutex
            .into_iter()
            .find(|pa| pa.lock().unwrap().start_page_index == page_list_start_index)
        {
            Some(page_list_attribute) => page_list_attribute,
            None => {
                return Err(Error::new(
                    ErrorType::Unexpected,
                    "plc_append_bad_page_list_start_index",
                    Some("Page index not found".to_string()),
                ))
            }
        };
        let mut page_list_attribute = page_list_attribute_mutex.lock().unwrap();

        let tail_page_void_size =
            page_list_attribute.last_page_void_size(page_store.page_settings.page_capacity());

        let (new_page_count, new_page_data_chunks) = make_chunks(
            &data[tail_page_void_size..],
            PageListAttributes::page_data_chunk_capacity(page_store.page_settings.page_capacity()),
        );

        // Allocate page_index to write data
        let new_page_index_list = page_store.get_page_indexes_for_writes(new_page_count)?;

        // Add next_page_index to payload of eachpage
        let page_index_payload_map =
            new_page_data_chunks
                .into_iter()
                .enumerate()
                .map(|(i, data)| {
                    let page_index = new_page_index_list[i];
                    let next_page_index = match new_page_count - i {
                        1 => u32::MAX as usize, // last page
                        _ => new_page_index_list[i + 1],
                    };
                    (page_index, next_page_index, data)
                });

        // Update new_last_page_data's next_page_index attribute, with index for 1st new page
        let tail_page_data_update = [
            page_list_attribute.tail_page_data_cache.clone(),
            data[..tail_page_void_size].to_vec(),
        ]
        .concat();
        let tail_page_next_page_index_update = match new_page_index_list.first() {
            Some(next_page_index) => *next_page_index,
            None => u32::MAX as usize,
        };
        let tail_page_payload_update = [
            tail_page_next_page_index_update.to_le_bytes().to_vec(),
            tail_page_data_update,
        ]
        .concat();

        // Write data to storage
        for (page_index, next_page_index, data) in page_index_payload_map.clone() {
            let page_payload: Vec<u8> =
                [next_page_index.to_le_bytes().to_vec(), data.to_vec()].concat();
            page_store.write_page(page_index, &page_payload, false)?;
        }
        page_store.write_page(
            page_list_attribute.tail_page_index,
            &tail_page_payload_update,
            false,
        )?;

        // Update tail_page_cache
        match page_index_payload_map.last() {
            Some((page_index, _, data)) => {
                page_list_attribute.update_tail_page_cache(page_index, data.clone())
            }
            None => {}
        }
        Ok(())
    }
}
