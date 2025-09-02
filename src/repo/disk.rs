use super::*;
use crate::index::IndexImp;
use crate::index::hash::HashIndex;
use std::{fs::File, path::PathBuf};
pub type Result<T> = std::result::Result<T, std::io::Error>;
struct DiskRepo {
    path: PathBuf,
    file: File,
    index: dyn IndexImp,
}
impl DiskRepo {
    fn new(path: PathBuf) -> Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let index = HashIndex::new(10);
        Ok(Self { path, file, index })
    }
}

impl Repo for DiskRepo {
    fn del(&self, req: DelRequest) -> bool {}
    fn get(&self, req: GetRequest) -> Option<String> {}
    fn put(&self, req: PutRequest) -> bool {}
}
pub fn from_file(path: PathBuf) -> Result<DiskRepo> {
    DiskRepo::new(path)
}
