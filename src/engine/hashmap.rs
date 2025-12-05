use super::*;
use std::collections::HashMap;
pub type Result<T> = std::result::Result<T, Box<std::io::Error>>;

pub struct MemoryHashMap(HashMap<Vec<u8>, Vec<u8>>);

impl Engine for MemoryHashMap {
    type Output = String;
    fn del(&mut self, req: DelRequest) -> bool {
        self.0.remove(&req.key).map_or(false, |_| true)
    }

    fn get(&self, req: GetRequest) -> Option<String> {
        self.0
            .get(&req.key)
            .map(|x| String::from_utf8_lossy(&x).to_string())
    }
    fn get_mut(&mut self, req: GetRequest) -> Option<String> {
        self.get(req)
    }
    fn put(&mut self, req: PutRequest) -> bool {
        if let Some(v) = req.value {
            self.0.insert(req.key, v);
            true
        } else {
            false
        }
    }
}
pub fn new() -> Result<MemoryHashMap> {
    Ok(MemoryHashMap(HashMap::new()))
}
