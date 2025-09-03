use super::*;
use crate::index::IndexImp;
use crate::index::hash::HashIndex;
use std::{
    fs::File,
    io::{BufWriter, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};
pub type Result<T> = std::result::Result<T, Box<std::io::Error>>;
pub struct DiskRepo {
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
        let value_len = value.len() as i32;
        entry.extend_from_slice(&key_len.to_le_bytes());
        entry.extend_from_slice(&value_len.to_le_bytes());

        // Append key and value as bytes
        entry.extend_from_slice(key.as_bytes());
        entry.extend_from_slice(value.as_bytes());

        entry
    }
    fn format_delete_entry(&self, key: &String) -> Vec<u8> {
        let mut entry = Vec::new();

        // Convert key length and value length to bytes and append
        let key_len = key.len() as u32;
        let value_len = -1 as i32;
        entry.extend_from_slice(&key_len.to_le_bytes());
        entry.extend_from_slice(&value_len.to_le_bytes());
        entry.extend_from_slice(key.as_bytes());
        entry
    }
    fn write_entry(
        &mut self,
        key: &String,
        value: &String,
        delete_mark: bool,
    ) -> Result<(u64, u32)> {
        let entry: Vec<u8>;
        match delete_mark {
            false => {
                // write delete entry
                entry = self.format_entry(key, value);
            }
            true => {
                // entry struct key.len()+value.len()+key+val
                entry = self.format_delete_entry(key);
            }
        }
        let offset = self.file.seek(SeekFrom::End(0))?;
        let mut writer = BufWriter::with_capacity(entry.len(), &mut self.file);
        println!("write entry: {:?}", entry);
        writer.write_all(&entry)?;
        writer.flush()?;
        Ok((offset, entry.len() as u32))
    }
    fn get_value_from_entry(&mut self, key: &String, offset: u64, length: u32) -> Result<String> {
        let mut entry = Vec::<u8>::with_capacity(length as usize);
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(&mut entry)?;
        println!("get entry: {:?}", entry);
        let value_len = i32::from_le_bytes(entry[4..8].try_into().unwrap());
        if value_len == -1 {
            return Ok("!Deleted".to_string());
        }
        // Todo assert entry[8..8+keylength] is key.as_bytes()
        let value_offset = length - 8 - u32::from_le_bytes(entry[4..8].try_into().unwrap());
        Ok(String::from_utf8_lossy(&entry[value_offset as usize..]).to_string())
    }
}

impl Repo for DiskRepo {
    fn del(&mut self, req: DelRequest) -> bool {
        // 插入一条特殊记录标记为已删除
        let delete_marker = String::new(); // 使用空字符串作为删除标记
        match self.write_entry(&req.key, &delete_marker, true) {
            Ok(_) => {
                // 更新索引，标记删除
                self.index.unset(&req.key);
                println!("Key '{}' marked as deleted.", req.key);
                true
            }
            Err(e) => {
                println!("Failed to mark key '{}' as deleted: {:?}", req.key, e);
                false
            }
        }
    }

    fn get(&mut self, req: GetRequest) -> Option<String> {
        // 从索引中查找键的偏移量和长度
        if let Some((offset, length)) = self.index.get(&req.key) {
            println!("offset:{},length:{}", offset, length);
            self.get_value_from_entry(&req.key, offset, length).ok();
        }
        None
    }

    fn put(&mut self, req: PutRequest) -> bool {
        let key_bor = &req.key;
        let Some(value_bor) = &req.value else {
            return false;
        };
        match self.write_entry(key_bor, &value_bor, false) {
            Ok((offset, entry_length)) => {
                self.index.set(key_bor, offset, entry_length);
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
