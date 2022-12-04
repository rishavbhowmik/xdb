A key value data base stores values against keys (Each key point to a value), Obviously.

### What do we need to build it?

1. A storage to quickly write values, so that we can read them quickly later. Writing data to a storage will give us an address, which can be used to locate & read the data.
2. Indexes to map keys with address pointing to location of value stored in storage.
3. A storage to persist indexes, so that everytime RAM looses power, we don't loose our KV mapping.
4. A parser to read persisted index from storage.

### How to make it usable?

We make an HTTP server, to send queries to the DB.

Queries are simple, we can use them to write, read & delete, key-value tupples in the DB.

No need to worry about tables for the sake of this POC.

We will learn some concurrency stuff here.

### What do we need from this experiment?

- [ ] Is `page_store` usable for an actual database with concurrent operations.
- [ ] A good roadmap for concurreny architechture, Future vrs combination of raw multithreading and channels.

## How to use for now?

Just run

```sh
cargo run -p kv-db
```

And ping the server

```sh
curl -X POST \
  'http://127.0.0.1:7878/hello' \
  --header 'Content-Type: application/json' \
  --data-raw '{ "hello": "world" }'
```

## Usage expectation

### Add new KV pair

```js
curl -X PUT \
  'http://127.0.0.1:7878/<COLLECTION_LOCATION>/<KEY>' \
  --data-raw '<VALUE>'
```

> use PUT or UPDATE

### Remove Key

```js
curl -X DELETE \
  'http://127.0.0.1:7878/<COLLECTION_LOCATION>/<KEY>' \
```

### Get value stored against a key

```js
curl -X GET \
  'http://127.0.0.1:7878/<COLLECTION_LOCATION>/<KEY>' \
  --data-raw '<VALUE>'
```
