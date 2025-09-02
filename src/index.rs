use crate::repo::Repo;

pub trait IndexImp {
    fn from_repo(repo: &dyn Repo) -> Self
    where
        Self: Sized;
}
pub mod hash;
