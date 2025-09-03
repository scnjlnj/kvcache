use super::*;
use crate::index::IndexImp;
use crate::index::hash::HashIndex;
use std::{
    fs::File,
    io::{BufWriter, Seek, Write},
    path::PathBuf,
};
pub type Result<T> = std::result::Result<T, Box<std::io::Error>>;
struct DiskRepo {
    path: PathBuf,
    file: File,
    index: Box<dyn IndexImp>,
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
        let index = Box::new(HashIndex::new(10));
        Ok(Self { path, file, index })
    }
    fn format_entry(&self, key: &String, value: &String) -> Vec<u8> {
        let mut entry = Vec::new();

        // Convert key length and value length to bytes and append
        let key_len = key.len() as u32;
        let value_len = value.len() as u32;
        entry.extend_from_slice(&key_len.to_le_bytes());
        entry.extend_from_slice(&value_len.to_le_bytes());

        // Append key and value as bytes
        entry.extend_from_slice(key.as_bytes());
        entry.extend_from_slice(value.as_bytes());

        entry
    }
    fn write_entry(&mut self, key: &String, value: &String) -> Result<(u64, u32)> {
        // entry struct key.len()+value.len()+key+val
        let entry = self.format_entry(key, value);
        let offset = self.file.seek(std::io::SeekFrom::End(0))?;
        let mut writer = BufWriter::with_capacity(entry.len(), &mut self.file);
        writer.write_all(&entry)?;
        writer.flush()?;
        Ok((offset, entry.len() as u32))
    }
}

impl Repo for DiskRepo {
    fn del(&mut self, req: DelRequest) -> bool {
        // 插入一条特殊记录标记为已删除
        let delete_marker = String::new(); // 使用空字符串作为删除标记
        match self.write_entry(&req.key, &delete_marker) {
            Ok((offset, entry_length)) => {
                // 更新索引，标记删除
                self.index.update(&req.key, offset, entry_length);
                println!("Key '{}' marked as deleted.", req.key);
                true
            }
            Err(e) => {
                println!("Failed to mark key '{}' as deleted: {:?}", req.key, e);
                false
            }
        }
    }

    fn get(&self, req: GetRequest) -> Option<String> {
        // 从索引中查找键的偏移量和长度
        if let Some((offset, length)) = self.index.get(&req.key) {
            let mut buffer = vec![0; length as usize];
            if let Ok(_) = self.file.seek(std::io::SeekFrom::Start(offset)) {
                if let Ok(_) = self.file.read_exact(&mut buffer) {
                    // 跳过 key 长度和 value 长度字段，直接读取 value
                    let key_len = u32::from_le_bytes(buffer[0..4].try_into().unwrap()) as usize;
                    let value_len = u32::from_le_bytes(buffer[4..8].try_into().unwrap()) as usize;
                    let value_start = 8 + key_len;
                    let value_end = value_start + value_len;
                    if value_end <= buffer.len() {
                        let value =
                            String::from_utf8_lossy(&buffer[value_start..value_end]).to_string();
                        // 如果值为空字符串，表示已删除
                        if value.is_empty() {
                            println!("Key '{}' is marked as deleted.", req.key);
                            return None;
                        }
                        return Some(value);
                    }
                }
            }
        }
        None
    }

    fn put(&mut self, req: PutRequest) -> bool {
        let key_bor = &req.key;
        let Some(value_bor) = &req.value else {
            return false;
        };
        match self.write_entry(key_bor, &value_bor) {
            Ok((offset, entry_length)) => {
                self.index.update(key_bor, offset, entry_length);
                return true;
            }
            Err(e) => {
                println!("{:?}", e);
                return false;
            }
        }
    }
}
pub fn from_file(path: PathBuf) -> Result<DiskRepo> {
    DiskRepo::new(path)
}
