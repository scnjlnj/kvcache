use crate::index;
use crate::repo;
use crate::repo::Repo;

#[test]
fn test_repo_disk() {
    let key = "a";
    let value = Some("abcd".to_string());
    let path = std::env::temp_dir().join("data").join("kv.db");
    let repo = repo::disk::from_file(path).unwrap();
    repo.put(key, value);
    let v = repo.get(key);
    assert_eq!(v, value);
}
