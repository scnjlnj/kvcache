
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
    fn put(&mut self, req: PutRequest) -> bool;

    // Get a value from the repository using a locator
    fn get(&mut self, req: GetRequest) -> Option<String>;

    // Delete a key-value pair from the repository
    fn del(&mut self, req: DelRequest) -> bool;
    fn iter_all(&self) -> Box<dyn Iterator<Item = (String, (u64, u32))>> {
        Box::new(Vec::<(String, (u64, u32))>::new().into_iter())
    }
}

pub mod disk;
