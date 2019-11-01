# RocksDB

Small demo app showing how to read/write/delete a key/value pair from RocksDB in Rust. Tested on MacOS

## Installation

First, install rocksdb with homebrew.

```
brew install rocksdb
```

Next, clone the repo.

```
git clone https://github.com/cgcardona/rocksdb.git
```

Then, change directories

```
cd rocksdb
```

Now, build the app and deps

```
cargo build
```

## Usage

To write

```
db.put("key", "value");
```

To read

```
db.get("key");
```

To delete

```
db.delete("key");
```

Lastly, run the app

```
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/rocksdb`
Writing key: foo and value: bar to rocksdb
Reading key: foo and value: bar from rocksdb
Deleting key: foo and value: bar from rocksdb
```

## Credits

This is using the [rust-rocksdb](https://github.com/rust-rocksdb/rust-rocksdb) crate.
