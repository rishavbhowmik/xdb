# Index Engine

An index is keys and values mapped in B-Tree or Hash-Map.

## Types of Indexes

1. B-Tree Index: A B-Tree is a self-balancing binary search tree, which also supports range searches.
2. Hash-Map Index: A Hash-Map is a data structure that maps keys to values.

## Persisting of Indexes in storage

Things to ensure while persisting index in storage:

- Index must be persisted for every write(INSERT/REMOVE/DELETE) operation.
- Any failed operation must be retrievable.

### Index Log

The Index log file serially records all write operations on the index.

#### Tupple format to insert a key and value pair

```
|-------------------------------|
| ENUM_INSERT          <1 Byte> |
|-------------------------------|--------------------|---------
| KEY_LEN             <4 Bytes> | <- Length of key   |
|-------------------------------|                    | <- Key
| KEY_DATA      <KEY_LEN Bytes> | <- Key data        |
|-------------------------------|--------------------|---------
| VALUE_LEN           <4 Bytes> | <- Length of value |
|-------------------------------|                    | <- Value
| VALUE     <4 * IND_LEN Bytes> | <- Array of indexs |
|-------------------------------|--------------------|---------
```

#### Tupple format to remove a key and value pair

```
|-------------------------------|
| ENUM_REMOVE          <1 Byte> |
|-------------------------------|--------------------|---------
| KEY_LEN             <4 Bytes> | <- Length of key   |
|-------------------------------|                    | <- Key
| KEY_DATA      <KEY_LEN Bytes> | <- Key data        |
|-------------------------------|--------------------|---------
| VALUE_LEN           <4 Bytes> | <- Length of value |
|-------------------------------|                    | <- Value
| VALUE     <4 * IND_LEN Bytes> | <- Array of indexs |
|-------------------------------|--------------------|---------
```

#### Tupple format to delete a key

```
|-------------------------------|
| ENUM_DELETE          <1 Byte> |
|-------------------------------|--------------------|---------
| KEY_LEN             <4 Bytes> | <- Length of key   |
|-------------------------------|                    | <- Key
| KEY_DATA      <KEY_LEN Bytes> | <- Key data        |
|-------------------------------|--------------------|---------
```

## Usage

### Quick Example

```rs
let mut btree_index = index::BTreeIndex::from_bytes(&[]).unwrap();

// insert a key and value pair
let insert_result = btree_index.insert(
    "One Plus One".as_bytes().to_vec(),
    "Two".as_bytes().to_vec()
);

// tuple for inserting the key and value pair
let insert_tuple_bytes = insert_result.unwrap();

// Now, append `insert_tuple_bytes` into index log
```

But we need a more reliable flow in a multi-threaded environment.

### Flow of operations

#### Create an index with Arc lock

```rs
use std::sync::{Arc, Mutex};

let mut btree_index_lock = Arc::new(
    Mutex::new(
        index::UniqueBTreeIndex::from_bytes(&[]).unwrap()
    )
);
```

#### Thread for write operations

```rs
enum WRITE_ENUM {
    INSERT,
    REMOVE,
    DELETE,
}

type UUID = u64;

/// channel for sending wite operation to the write thread
type IndexWriteChanPayload = (UUID, WRITE_ENUM, Vec<u8>, Vec<u8>)
let (index_write_tx, index_write_rx): (Sender<IndexWriteChanPayload>, Receiver<IndexWriteChanPayload>) = channel();

/// Channel for receiving results from the write thread
type IndexWriteResChanPayload = (UUID, Vec<u8>);
let (index_write_res_tx, index_write_res_rx): (Sender<IndexWriteResChanPayload>, Receiver<IndexWriteResChanPayload>) = channel();

/// btree_index_lock clone for write thread
let btree_index_lock_clone = btree_index_lock.clone();

let write_thread = thread::spawn(move || {
    // listen to index_write channel
    loop {
        if let Ok(write_tuple) = index_write_rx.recv() {
            let mut btree_index_lock = btree_index_lock.lock().unwrap();

            let uuid = write_tuple.0;

            let result = match write_tuple.1 {
                WRITE_ENUM::INSERT => btree_index_lock.insert(write_tuple.2, write_tuple.3),
                WRITE_ENUM::REMOVE => btree_index_lock.remove(write_tuple.2, write_tuple.3),
                WRITE_ENUM::DELETE => btree_index_lock.delete(write_tuple.2),
            };

            // send result to the main thread
            if result.is_ok() {
                index_write_res_tx.send((uuid, result.unwrap())).unwrap();
            } else {
                index_write_res_tx.send((uuid, vec![])).unwrap();
            }
        }
    }
});

// insert a key and value pair
let common_uuid = 11 as UUID;
index_write_tx.send((common_uuid, WRITE_ENUM::INSERT, "One Plus One".as_bytes().to_vec(), "Two".as_bytes().to_vec())).unwrap();
if let Ok(write_res) = index_write_res_rx.recv() {
    assert_eq!(write_res.0, common_uuid);
    assert!(write_res.1.len());
}
```

#### INSERT

1. Lock all read and write operations.
2. Insert new key-value pair in the index.
3. Append index log with `INSERT_LOG` tuple.
4. Unlock all read and write operations.

#### REMOVE

1. Lock all read and write operations.
2. Remove the key-value pair in the index.
3. Append index log with `REMOVE_LOG` tuple.
4. Unlock all read and write operations.

#### DELETE

1. Lock all read and write operations.
2. Delete key from the index.
3. Append index log with `DELETE_LOG` tuple.
4. Unlock all read and write operations.

#### READ

Just read it!

### Spawning index from existing index log bytes

```rs
let index_sync_bytes: Vec<u8> = vec![];

/// create new index
let mut new_index = index::BTreeIndex::from_bytes(&index_sync_bytes).unwrap();

/// Insert some key value pairs
new_index.insert(b"One Plus One".to_vec(), b"Two".to_vec());
new_index.insert(b"Two Plus Two".to_vec(), b"Four".to_vec());

// "One Plus One" -> "Two", "Two Plus Two" -> "Four"

/// Spawn index from index log
let spawned_index = index::BTreeIndex::from_bytes(&index_sync_bytes).unwrap();

/// spawned_index is identical to new_index
assert_eq!(spawned_index.index_clone(), new_index.index_clone());
```
