use rkyv::{Archive, Deserialize, Serialize};

/// 将不同的数据包装成同一形式方面传递
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum Data {
    DocBucket(DocBucket),
}

/// 文档桶: 在内存中累积多篇文章, 达到设定大小后整体写入磁盘的缓冲区
/// impl见[bucket.rs](/storage/bucket.rs)
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct DocBucket {
    /// 桶的ID
    pub id: u64,
    /// 桶的大小(用存储文章的大小计算,实际可能更大)
    pub total_size: u64,
    /// 存储文章的内容
    pub content: Vec<String>,
}
