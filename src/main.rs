use rocksdb::DB;

fn main() {
    let path: &str = "tmp/rust-rocksdb";
    let db: rocksdb::DB = DB::open_default(path).unwrap();

    let key: &str = "foo";
    let value: &str = "bar";

    assert!(db.put(key, value).is_ok());

    match db.get(key) {
        Ok(Some(value)) => match value.to_utf8() {
            Some(v) => println!("retrieved utf8 value: {}", v),
            None => println!("did not read valid utf-8 out of the db"),
        },
        Ok(None) => panic!("value not present!"),
        Err(e) => println!("error retrieving value: {}", e),
    }

    assert!(db.delete(key).is_ok());
}
