use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum Data {
    DocBucket(DocBucket),
}

/// impl见[bucket.rs](/storage/bucket.rs)
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct DocBucket {
    pub id: u64,
    pub total_size: u64,
    pub content: Vec<String>,
}
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct Document {
    pub id: u64,
    pub content: String,
}
