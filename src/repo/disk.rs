use super::*;
use crate::index::IndexImp;
use crate::index::hash::HashIndex;
use std::{
    fs::File,
    io::{BufWriter, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};
pub type Result<T> = std::result::Result<T, Box<std::io::Error>>;
#[derive(Debug)]
pub struct Entry {
    pub key: String,
    pub offset: u64,
    pub length: u32,
    pub deleted: bool,
}
pub struct EntryIter<'a> {
    file: &'a mut File,
    offset: u64,
    file_len: u64,
}
impl<'a> EntryIter<'a> {
    pub fn new(file: &'a mut File) -> std::io::Result<Self> {
        let file_len = file.metadata()?.len();
        Ok(Self {
            file,
            offset: 0,
            file_len,
        })
    }
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.file_len {
            return None;
        }

        // 读 header
        let mut header = [0u8; 8];
        if self.file.seek(SeekFrom::Start(self.offset)).is_err() {
            return None;
        }
        if self.file.read_exact(&mut header).is_err() {
            return None;
        }

        let key_len = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let value_len = i32::from_le_bytes(header[4..8].try_into().unwrap());

        let entry_length = 8 + key_len as u32 + if value_len >= 0 { value_len as u32 } else { 0 };

        // 读 key
        let mut key_buf = vec![0u8; key_len as usize];
        if self.file.read_exact(&mut key_buf).is_err() {
            return None;
        }
        let key = String::from_utf8_lossy(&key_buf).to_string();

        // 跳过 value 部分
        if value_len > 0 {
            let mut dummy = vec![0u8; value_len as usize];
            let _ = self.file.read_exact(&mut dummy);
        }

        let entry = Entry {
            key,
            offset: self.offset,
            length: entry_length,
            deleted: value_len == -1,
        };

        self.offset += entry_length as u64;
        Some(entry)
    }
}
pub struct DiskRepo {
    path: PathBuf,
    file: File,
    index: Box<dyn IndexImp>,
}
impl DiskRepo {
    pub fn iter_entries(&mut self) -> std::io::Result<EntryIter<'_>> {
        EntryIter::new(&mut self.file)
    }
    fn new(path: PathBuf) -> Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let mut index = Box::new(HashIndex::new(10));
        // 用迭代器扫描文件，恢复索引
        {
            // 用迭代器扫描文件，恢复索引
            let iter = EntryIter::new(&mut file)?;
            for entry in iter {
                if entry.deleted {
                    index.unset(&entry.key);
                } else {
                    index.set(&entry.key, entry.offset, entry.length);
                }
            }
        }
        Ok(Self { path, file, index })
    }
    fn parse_entry_at(&mut self, offset: u64) -> Option<(String, u64, u32)> {
        // 先读 8 字节 header
        let mut header = [0u8; 8];
        if self.file.seek(SeekFrom::Start(offset)).is_err() {
            return None;
        }
        if self.file.read_exact(&mut header).is_err() {
            return None;
        }

        let key_len = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let value_len = i32::from_le_bytes(header[4..8].try_into().unwrap());

        // 计算 entry 总长度
        let entry_length = 8 + key_len as u32 + if value_len >= 0 { value_len as u32 } else { 0 };

        // 读 key
        let mut key_buf = vec![0u8; key_len as usize];
        if self.file.read_exact(&mut key_buf).is_err() {
            return None;
        }
        let key = String::from_utf8_lossy(&key_buf).to_string();

        // value 部分跳过或读掉
        if value_len >= 0 {
            let mut val_buf = vec![0u8; value_len as usize];
            let _ = self.file.read_exact(&mut val_buf);
        }

        Some((key, offset, entry_length))
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
        writer.write_all(&entry)?;
        writer.flush()?;
        Ok((offset, entry.len() as u32))
    }
    fn get_value_from_entry(&mut self, key: &String, offset: u64, length: u32) -> Result<String> {
        let mut entry = vec![0u8; length as usize];
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(&mut entry)?;
        let value_len = i32::from_le_bytes(entry[4..8].try_into().unwrap());
        if value_len == -1 {
            return Ok("!Deleted".to_string());
        }
        // Todo assert entry[8..8+keylength] is key.as_bytes()
        let value_offset = 8 + u32::from_le_bytes(entry[0..4].try_into().unwrap());
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
            return self.get_value_from_entry(&req.key, offset, length).ok();
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
