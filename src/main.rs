use kvcache::repo;

fn main() {
    let path = std::env::temp_dir().join("data").join("kv.db");
    println!("{:?}", path);
    let mut repo = repo::disk::from_file(path).unwrap();
    // show all entries
    for entry in repo.iter_entries().unwrap() {
        println!("{:?}", entry);
    }
}
