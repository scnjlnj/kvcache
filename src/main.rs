use kvcache::engine;

fn main() {
    let path = std::env::temp_dir().join("data").join("kv.db");
    println!("{:?}", path);
    let mut repo = engine::bitcask::from_file(path).unwrap();
    // show all entries
    for entry in repo.iter_entries().unwrap() {
        println!("{:?}", entry);
    }
}
