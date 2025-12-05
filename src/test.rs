#[cfg(test)]
mod test_engine {
    use std::collections::HashMap;

    use random_word::Lang;

    use crate::engine;
    use crate::engine::Engine;
    use crate::engine::GetRequest;
    use crate::engine::PutRequest;
    use crate::engine::ReadMutEngine;
    #[cfg(test)]
    mod test_bitcask {
        use super::*;
        #[test]
        fn test_engine_disk() {
            let path = std::env::temp_dir().join("data").join("kv.db");
            println!("{:?}", path);
            let mut engine = engine::bitcask::from_file(path).unwrap();
            let put = PutRequest::new("a", Some("abcd".to_string().into_bytes()));
            engine.put(put);
            let get = GetRequest::new("a");
            let v = engine.get(get);
            println!("get: {:?}", v);
            assert_eq!(v, Some("abcd".to_string()));
        }

        #[test]
        fn test_engine_disk_2() {
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
            let mut engine = engine::bitcask::from_file(path).unwrap();
            for (k, v) in words.iter() {
                let put = PutRequest::new(*k, Some(v.to_string().into_bytes()));
                engine.put(put);
            }
            // check
            for (k, v) in words.iter() {
                let get = GetRequest::new(*k);
                let vv = engine.get(get);
                assert_eq!(vv, Some(v.to_string()));
            }
        }

        #[test]
        fn test_engine_disk_build_index() {
            // clear file and insert data
            let path = std::env::temp_dir().join("data").join("kv.db");
            {
                // delete os file if exist
                if path.exists() {
                    std::fs::remove_file(path.clone()).unwrap();
                }
                let mut engine = engine::bitcask::from_file(path.clone()).unwrap();
                let insert_data = vec![("key1", "value1"), ("key2", "value2"), ("key3", "value3")];
                for (k, v) in insert_data {
                    let put = PutRequest::new(k, Some(v.to_string().into_bytes()));
                    engine.put(put);
                }
            }
            // query last inserted data
            {
                let mut engine = engine::bitcask::from_file(path.clone()).unwrap();
                let insert_data = vec![("key1", "value1"), ("key2", "value2"), ("key3", "value3")];
                for (k, v) in insert_data {
                    let get = GetRequest::new(k);
                    let vv = engine.get(get);
                    assert_eq!(vv, Some(v.to_string()));
                }
            }
        }
    }
    #[cfg(test)]
    mod test_hash {
        use super::*;

        #[test]
        fn test_engine_mem_hash_0() {
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

            let mut engine = engine::hashmap::new().unwrap();
            for (k, v) in words.iter() {
                let put = PutRequest::new(*k, Some(v.to_string().into_bytes()));
                engine.put(put);
            }
            // check
            for (k, v) in words.iter() {
                let get = GetRequest::new(*k);
                let vv = engine.get(get);
                assert_eq!(vv, Some(v.to_string()));
            }
        }
    }
    #[cfg(test)]
    mod test_btree {
        use std::{
            sync::{Arc, RwLock},
            thread,
        };

        use super::*;

        #[test]
        fn test_engine_mem_btree_0() {
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

            let mut engine = engine::btree::new().unwrap();
            for (k, v) in words.iter() {
                let put = PutRequest::new(*k, Some(v.to_string().into_bytes()));
                engine.put(put);
            }
            // check
            for (k, v) in words.iter() {
                let get = GetRequest::new(*k);
                let vv = engine.get(get);
                assert_eq!(vv, Some(v.to_string()));
            }
        }
        #[test]
        fn test_multi_thread() {
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

            let engine = Arc::new(RwLock::new(engine::btree::new().unwrap()));
            let mut handlers = Vec::new();
            for (k, v) in words.iter() {
                let put = PutRequest::new(*k, Some(v.to_string().into_bytes()));
                let engine = Arc::clone(&engine);
                let h = thread::spawn(move || {
                    engine.write().unwrap().put(put);
                });
                handlers.push(h);
            }
            for h in handlers {
                h.join().unwrap();
            }
            // check
            for (k, v) in words.iter() {
                let get = GetRequest::new(*k);
                let vv = engine.read().unwrap().get(get);
                assert_eq!(vv, Some(v.to_string()));
            }
        }
    }
}
