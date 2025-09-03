# KVCache

KVCache 是一个使用rust实现的基于磁盘的键值存储系统，支持基本的增删查操作，并通过索引优化查询性能。是作为学习Rust的练手项目

## 功能特性

- **持久化存储**：数据存储在磁盘文件中，支持重启后数据恢复。
- **高效索引**：使用 `HashIndex` 提供快速的键值查找。
- **基本操作**：
  - `put`：插入或更新键值对。
  - `get`：根据键获取值。
  - `del`：标记键值对为已删除。

## 文件结构
```
src
├── bitcask.rs
├── index
│   └── hash.rs
├── index.rs
├── lib.rs
├── main.rs
├── repo
│   └── disk.rs
├── repo.rs
└── test.rs

```

## 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/your-username/kvcache.git
cd kvcache
```
### 2. 构建和测试
- 确保您已安装 Rust 工具链
`cargo test -- --nocapture`
## 希望实现的功能
- [Done] 使用std::collections::HashMap作为默认内存索引
- [Done] WAL和Bitcask存储和Replay 迭代器
- std::thread 版本实现
- tokio 版本实现
- 实现btree内存索引
- 大Key优化节省内存 （bigkey,value） => (hash(bigkey,256),value) + 碰撞兜底策略（没想好）
- 压缩节省内存和磁盘
- 

## 参考
- https://github.com/rosedblabs/rust-practice
- https://github.com/erikgrinaker/toydb
- https://github.com/lancedb/lancedb
