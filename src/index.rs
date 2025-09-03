use crate::repo::Repo;

pub trait IndexImp {
    fn from_repo(repo: &dyn Repo) -> Self
    where
        Self: Sized;
    fn update(&mut self, key: &String, offset: u64, entry_len: u32) -> bool;
}
pub mod hash;
