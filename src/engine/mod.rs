// A request to get entry by Locator
pub struct GetRequest {
    pub key: Vec<u8>,
}

impl GetRequest {
    // 创建一个新的 GetRequest
    pub fn new(key: impl Into<Vec<u8>>) -> Self {
        Self { key: key.into() }
    }
}

// A request to put k/v
pub struct PutRequest {
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
}

impl PutRequest {
    // 创建一个新的 PutRequest
    pub fn new(key: impl Into<Vec<u8>>, value: impl Into<Option<Vec<u8>>>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    // 检查是否有值
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
}

// A request to delete an entry
pub struct DelRequest {
    pub key: Vec<u8>,
}

impl DelRequest {
    // 创建一个新的 DelRequest
    pub fn new(key: impl Into<Vec<u8>>) -> Self {
        Self { key: key.into() }
    }
}
pub trait Engine: Send {
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

pub mod bitcask;
