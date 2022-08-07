# PageStore

PageStore module is for storing pages in a storage file, and perform CRUD operations on the pages.

## What is a page?

A page is a storage unit of constant size (The fixed length of the page is `page_length`).

Each page can store data of size less or equal to the `page_length`.

### Terminologies

- `page_length`: The size of a page.
- `page_index`: The unique serial index of a page in the storage.
- `page_payload`: The data stored in a page. (excluding page header)
- `page_payload_length`: The size of the data payload of a page. (This is the page header)

### Layout of a page

```txt
| -------------------------------------- |
| page_payload_length as little endian   | <- Page header |
| -------------------------------------- |
| page_payload                           | <- Page data   |
| -------------------------------------- |
```

#### Size of `page_payload_length` little endian byte representation

`page_payload_length` is an unsigned integer stored in little-endian format. Since the maximum value of `page_payload_length` is always less than `page_length`, we have implemented adaptive byte representation to save storage space & reduce traversal time.

| Page length             | Unsigned integer type | Bytes required |
| ----------------------- | --------------------- | -------------- |
| <= 255                  | u8                    | 1              |
| <= 65535                | u16                   | 2              |
| <= 4294967295           | u32                   | 4              |
| <= 18446744073709551615 | u64                   | 8              |

> To implement unsigned integer with adaptive byte representation, we use `PageUsizeType`.
>
> Check `storage_engine/page_store/src/page_usize.rs` to find more.

## Layout of storage file

```txt
| ---------------------------------------------------- |----------------------- | 
| Page length        <8 Bytes>                         | <- Page store header    |
| ---------------------------------------------------- | ----------------------- | ----------------------- |
| page_payload_length as LE bytes                      | <- Page header          |                         |
| ---------------------------------------------------- | ----------------------- | <- Page index 0         |
| page_payload       <page_payload_length bytes>       | <- Page data            |                         |
| ---------------------------------------------------- | ----------------------- | ----------------------- |
| Block 2 dataSize <4 Bytes>                           | <- Block header         |                         |
| ---------------------------------------------------- | ----------------------- | <- Page index 1         |
| Block 2 Data    <BLOCK_LEN>                          | <- Block data           |                         |
| ---------------------------------------------------- | ----------------------- | ----------------------- |
| so on...                                             |
```

The first 8 bytes are reserved for storing u64 integer in little-endian format.

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
