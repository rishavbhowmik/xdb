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

#### INSERT

1. Lock all read and write operations.
2. Insert new key-value pair in the index.
3. Append index log with `INSERT_LOG` tuple.
4. Unlock all read and write operations.

#### REMOVE

1. Lock all read and write operations.
2. Remove the key-value pair in the index.
3. Append index log with `INSERT_LOG` tuple.
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
