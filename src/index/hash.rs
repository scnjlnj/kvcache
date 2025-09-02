use std::collections::HashMap;

use crate::index::IndexImp;

pub struct HashIndex {
    data: HashMap<String, String>,
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
        let data = repo.iter_all().collect::<HashMap<String, String>>();
        Self { data }
    }
}
