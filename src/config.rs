use std::path::PathBuf;

/// 存储文章桶的配置
pub struct DocBucketConfig {
    /// 每个已刷盘桶的最后一篇文章的ID(即该桶的结束ID), 按刷盘顺序排列
    /// 详细说明见[bucket.md](../doc/bucket.md#bucket_ranges)
    pub bucket_ranges: Vec<u64>,
    /// 一个桶的最大大小(写入时可能略大)
    pub threshold: u64,
    /// 已分配文章总数(即下一篇文章的ID), 写入时递增
    pub article_count: u64,
    /// 所有桶文件存储在此目录下
    pub path: PathBuf,
}

impl DocBucketConfig {
    pub fn new(path: PathBuf) -> Self {
        DocBucketConfig {
            bucket_ranges: Vec::new(),
            threshold: 64 * 1024 * 1024, // 64MB
            article_count: 0,
            path,
        }
    }

    /// 设置桶的最大大小
    pub fn set_threshold(&mut self, threshold: u64) {
        self.threshold = threshold;
    }
    /// 接收一个文章的ID, 然后返回一个元组 (桶的ID, 文章在桶内的索引).
    /// 详细说明见[bucket.md](../doc/bucket.md#get_index)
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

#[cfg(test)]
mod tests {
    use super::*;
    fn make_config(ranges: Vec<u64>) -> DocBucketConfig {
        DocBucketConfig {
            bucket_ranges: ranges,
            threshold: 0,
            article_count: 0,
            path: PathBuf::from("/tmp/test"),
        }
    }
    #[test]
    fn test_get_index_active_bucket() {
        let config = make_config(vec![30000, 67100]);
        // 文章67101应在活跃桶(桶2)中, 偏移=67101-67100-1=0
        assert_eq!(config.get_index(67101), (2, 0));
        // 文章99999应在活跃桶中，偏移=99999-67100-1=32898
        assert_eq!(config.get_index(99999), (2, 99999 - 67100 - 1));
    }
}
