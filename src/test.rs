use std::collections::HashMap;

use random_word::Lang;

use crate::engine;
use crate::engine::Engine;
use crate::engine::GetRequest;
use crate::engine::PutRequest;

#[test]
fn test_repo_disk() {
    let path = std::env::temp_dir().join("data").join("kv.db");
    println!("{:?}", path);
    let mut repo = engine::bitcask::from_file(path).unwrap();
    let put = PutRequest::new("a", Some("abcd".to_string().into_bytes()));
    repo.put(put);
    let get = GetRequest::new("a");
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
    let mut repo = engine::bitcask::from_file(path).unwrap();
    for (k, v) in words.iter() {
        let put = PutRequest::new(*k, Some(v.to_string().into_bytes()));
        repo.put(put);
    }
    // check
    for (k, v) in words.iter() {
        let get = GetRequest::new(*k);
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
        let mut repo = engine::bitcask::from_file(path.clone()).unwrap();
        let insert_data = vec![("key1", "value1"), ("key2", "value2"), ("key3", "value3")];
        for (k, v) in insert_data {
            let put = PutRequest::new(k, Some(v.to_string().into_bytes()));
            repo.put(put);
        }
    }
    // query last inserted data
    {
        let mut repo = engine::bitcask::from_file(path.clone()).unwrap();
        let insert_data = vec![("key1", "value1"), ("key2", "value2"), ("key3", "value3")];
        for (k, v) in insert_data {
            let get = GetRequest::new(k);
            let vv = repo.get(get);
            assert_eq!(vv, Some(v.to_string()));
        }
    }
}
