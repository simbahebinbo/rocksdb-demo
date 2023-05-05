fn main() {
    let path: &str = "tmp/rust-rocksdb";
    let db: rocksdb::DB = rocksdb::DB::open_default(path).unwrap();

    let key: &str = "foo";
    let value: &str = "bar";

    println!("Writing key: {} and value: {} to rocksdb", key, value);
    _ = db.put(key, value);

    match db.get(key) {
        Ok(Some(value)) => match String::from_utf8(value).ok() {
            Some(v) => println!("Reading key: {} and value: {} from rocksdb", key, v),
            None => println!("did not read valid utf-8 out of the db"),
        },
        Ok(None) => panic!("value not present!"),
        Err(e) => println!("error retrieving value: {}", e),
    }

    println!("Deleting key: {} and value: {} from rocksdb", key, value);
    _ = db.delete(key);
}
