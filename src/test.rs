use crate::index;
use crate::repo;
use crate::repo::GetRequest;
use crate::repo::PutRequest;
use crate::repo::Repo;

#[test]
fn test_repo_disk() {
    let path = std::env::temp_dir().join("data").join("kv.db");
    println!("{:?}", path);
    let mut repo = repo::disk::from_file(path).unwrap();
    let put = PutRequest {
        key: "a".to_string(),
        value: Some("abcd".to_string()),
    };
    repo.put(put);
    let get = GetRequest {
        key: "a".to_string(),
    };
    let v = repo.get(get);
    assert_eq!(v, Some("abcd".to_string()));
}
