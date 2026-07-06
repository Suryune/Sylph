use crate::config::DocBucketConfig;
use crate::data::{Data, DocBucket};
use crate::error::Result;
use crate::storage;
impl DocBucket {
    pub fn new(id: u64) -> Self {
        DocBucket {
            id,
            total_size: 0,
            content: Vec::new(),
        }
    }
    pub fn add_doc(&mut self, doc: String, config: &mut DocBucketConfig) -> Result<u64> {
        let len: u64 = doc.len() as u64;
        self.content.push(doc);
        self.total_size += len;
        let article_id = config.article_count;

        if self.total_size >= config.threshold {
            let bucket_id = config.bucket_ranges.len() as u64;
            let old_bucket = std::mem::replace(self, DocBucket::new(bucket_id));
            let data = Data::DocBucket(old_bucket);
            storage::write_data(&data, &config.path)?;
            config.bucket_ranges.push(article_id);
        }

        config.article_count += 1;
        Ok(article_id)
    }
}
