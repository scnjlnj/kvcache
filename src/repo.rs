use std::path::PathBuf;

use crate::index::IndexImp;
// A request to get entry by Locator
pub struct GetRequest {
    pub key: String,
}
// A request to put k/v
pub struct PutRequest {
    pub key: String,
    pub value: Option<String>,
}
// A request to delete a entry
pub struct DelRequest {
    pub key: String,
}

pub trait Locator {}

pub trait Repo {
    // Put a key-value pair into the repository
    fn put(&self, req: PutRequest) -> bool;

    // Get a value from the repository using a locator
    fn get(&self, req: GetRequest) -> Option<String>;

    // Delete a key-value pair from the repository
    fn del(&self, req: DelRequest) -> bool;

    fn iter_all(&self) -> impl IntoIterator<Item = (String, String)> {
        Vec::<(String, String)>::new()
    }
}

pub mod disk;
