use std::collections::HashMap;

use random_word::Lang;

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
    println!("get: {:?}", v);
    assert_eq!(v, Some("abcd".to_string()));
}
#[test]
fn test_repo_disk_2() {
    let words: HashMap<&str, &str> = random_word::all(Lang::Zh)
        .iter()
        .step_by(1024)
        .take(500)
        .copied()
        .filter_map(|s| {
            // split_once ==> Option<(&str, &str)>
            s.split_once(' ')
        })
        .collect();

    let path = std::env::temp_dir().join("data").join("kv.db");
    let mut repo = repo::disk::from_file(path).unwrap();
    for (k, v) in words.iter() {
        let put = PutRequest {
            key: k.to_string(),
            value: Some(v.to_string()),
        };
        repo.put(put);
    }
    // check
    for (k, v) in words.iter() {
        let get = GetRequest { key: k.to_string() };
        let vv = repo.get(get);
        assert_eq!(vv, Some(v.to_string()));
    }
}

#[test]
fn test_repo_disk_build_index() {
    // clear file and insert data
    let path = std::env::temp_dir().join("data").join("kv.db");
    {
        // delete os file if exist
        if path.exists() {
            std::fs::remove_file(path.clone()).unwrap();
        }
        let mut repo = repo::disk::from_file(path.clone()).unwrap();
        let insert_data = vec![("key1", "value1"), ("key2", "value2"), ("key3", "value3")];
        for (k, v) in insert_data {
            let put = PutRequest {
                key: k.to_string(),
                value: Some(v.to_string()),
            };
            repo.put(put);
        }
    }
    // query last inserted data
    {
        let mut repo = repo::disk::from_file(path.clone()).unwrap();
        let insert_data = vec![("key1", "value1"), ("key2", "value2"), ("key3", "value3")];
        for (k, v) in insert_data {
            let get = GetRequest { key: k.to_string() };
            let vv = repo.get(get);
            assert_eq!(vv, Some(v.to_string()));
        }
    }
}
