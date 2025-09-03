use crate::repo::Repo;

pub trait IndexImp {
    fn from_repo(repo: &dyn Repo) -> Self
    where
        Self: Sized;
    fn set(&mut self, key: &String, offset: u64, entry_len: u32) -> bool;
    fn unset(&mut self, key: &String) -> bool;
    fn get(&self, key: &String) -> Option<(u64, u32)>;
}
pub mod hash;
