use std::collections::HashMap;

use crate::index::IndexImp;

pub struct HashIndex {
    data: HashMap<String, (u64, u32)>,
}
impl HashIndex {
    pub fn new(cap: usize) -> Self {
        Self {
            data: HashMap::with_capacity(cap),
        }
    }
}

impl IndexImp for HashIndex {
    fn from_repo(repo: &dyn crate::repo::Repo) -> Self
    where
        Self: Sized,
    {
        let data = repo.iter_all().collect::<HashMap<String, (u64, u32)>>();
        Self { data }
    }
    fn set(&mut self, key: &String, offset: u64, entry_len: u32) -> bool {
        self.data.insert(key.clone(), (offset, entry_len));
        return true;
    }
    fn unset(&mut self, key: &String) -> bool {
        self.data.remove(key);
        return true;
    }
    fn get(&self, key: &String) -> Option<(u64, u32)> {
        self.data.get(key).map(|x| x.clone())
    }
}
