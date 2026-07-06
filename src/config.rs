use std::path::PathBuf;

pub struct DocBucketConfig {
    pub bucket_ranges: Vec<u64>,
    pub threshold: u64,
    pub article_count: u64,
    pub path: PathBuf,
}

impl DocBucketConfig {
    pub fn new(path: PathBuf) -> Self {
        DocBucketConfig {
            bucket_ranges: Vec::new(),
            threshold: 64 * 1024 * 1024,
            article_count: 0,
            path,
        }
    }
    pub fn set_threshold(&mut self, threshold: u64) {
        self.threshold = threshold;
    }
    pub fn get_index(&self, article_id: u64) -> (u64, u64) {
        // let left = self.bucket_ranges.partition_point(|&x| x < article_id);
        let mut left = 0;
        let mut right = self.bucket_ranges.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if self.bucket_ranges[mid] < article_id {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if left == 0 {
            (0, article_id)
        } else {
            (left as u64, article_id - self.bucket_ranges[left - 1] - 1)
        }
    }
}
