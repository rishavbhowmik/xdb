# PageStore

PageStore module is for storing pages in a storage file, and perform CRUD operations on the pages.

## What is a page?

A page is a storage unit of constant size (this constant size is `page_length`).

Each page can store data of size less or equal to `page_length`.

### Terminologies

- `page_length`: The size of a page.
- `page_index`: The unique serial index of a page in the storage.
- `page_payload`: The data stored in a page. (excluding page header)
- `page_payload_length`: The size of the data payload of a page. (This is the page header)

### Layout of a page

```txt
|--------------------------------------|
| page_payload_length as little endian   | <- Page header |
| -------------------------------------- |
| page_payload                           | <- Page data   |
| -------------------------------------- |
```

#### Size of `page_payload_length` little endian byte representation

The size this page header depends on the constant page length. We have made it adaptive to make storage more efficient.

| Page length             | Bytes required |
| ----------------------- | -------------- |
| <= 255                  | 1              |
| <= 65535                | 2              |
| <= 4294967295           | 4              |
| <= 18446744073709551615 | 8              |

To manage this unsigned integer with adaptive byte representation, we use `PageUsizeType`.

> Check `storage_engine/page_store/src/page_usize.rs` to find more.

## Layout of storage file

```txt
|----------------------------------------------------| ---------------------- | 
| Page length        <8 Bytes>                       | <- Page store header   |
| -------------------------------------------------- | ---------------------- | ---------------------- |
| page_payload_length as LE bytes                    | <- Page header         |                        |
| -------------------------------------------------- | ---------------------- | <- Page index 0        |
| page_payload       <page_payload_length bytes>     | <- Page data           |                        |
| --------------------------------------             | ---------------------- | ---------------------- |
| Block 2 dataSize <4 Bytes>                         | <- Block header        |                        |
| ----------------------------                       | ---------------------- | <- Page index 1        |
| Block 2 Data    <BLOCK_LEN>                        | <- Block data          |                        |
| -------------------------------------------------- | ---------------------- | ---------------------- |
| so on...                                           |
```

First 8 bytes are for constant page length, stored as little endian 64-bit unsigned integer. (This is not adaptive & requires 8 bytes)

## How to use PageStore

```rs
use storage_engine::page_store::PageStore;

const PAGE_LENGTH: usize = 1024;

fn main() {
    // Create a new storage file
    let mut page_store = PageStore::new("storage_file", PAGE_LENGTH);
    
    // Write data to page
    let data_list = [b"Hello World 1", b"Hello World 2"];
    let result = page_store.write_many_pages(&data_list);
    let page_index_list = result.unwrap();

    // Read data from page
    let page_0_result = page_store.read_page(page_index_list[0]);
    assert_eq!(page_0_result.unwrap(), data_list[0]);
    let page_1_result = page_store.read_page(page_index_list[1]);
    assert_eq!(page_1_result.unwrap(), data_list[1]);

    // Delete page
    let result = page_store.delete_page(page_index_list[0]);
    // Read deleted page & verify it is empty
    let page_0_result = page_store.read_page(page_index_list[0]);
    assert_eq!(page_0_result.unwrap(), vec![]);
}
```
