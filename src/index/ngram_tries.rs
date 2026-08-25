use crate::index::TreeNode;

/// 和 TermTrieNode 结构类似的 n-gram 树节点, 但增加了前缀与分支的存在标志
/// 数组布局:
/// [0]: prefix 值
/// [1]: prefix 标志位 (0: prefix 本身不作为独立 n-gram 存在, 1: 存在)
/// [2]: len (mid 数量 + 3，即元数据字段个数)
/// [3..len]: 按升序排列的 mid
/// [len..len+len-3]: 每个 mid 的起始索引 (其中第一个位置存放 mid 自身的标志位)
/// [len+len-3]: 总长度字段
/// 这种设计使得仅查询某个 mid 是否存在时, 不会因为已添加其子集而误判为存在
pub struct NgramTreeNode(pub Vec<u32>);

impl TreeNode for NgramTreeNode {
    fn insert(&mut self, mid: u32, postfix: Option<u32>) {
        let len = self.0[2] as usize; // [0..=1]为prefix和它的标志位, len变成了[2]
        let branches = &self.0[3..len];
        match branches.binary_search(&mid) {
            Ok(i) => {
                let i_index = len + i;
                let start = self.0[i_index] as usize;
                if let Some(last) = postfix {
                    let next_mid_index = i_index + 1;
                    let end = self.0[next_mid_index] as usize;
                    // 每个 mid 对应的起始索引位置存的是一个标志位 (0 或 1),
                    // 表示该 mid 本身是否作为一个独立的 n-gram 存在
                    match &self.0[start + 1..end].binary_search(&last) {
                        Ok(_) => (),
                        Err(insert_index) => {
                            self.0.insert(start + insert_index + 1, last);
                            let updated = &mut self.0[next_mid_index..len + len - 3];
                            for item in updated {
                                *item += 1;
                            }
                            // 总长度字段的索引为 (len + len - 3)
                            // 因为有 prefix及其标志位 和 len 3个元数据字段, 且len表示的为 mid的数量+3
                            self.0[len + len - 3] = self.0.len() as u32;
                        }
                    }
                } else {
                    self.0[start] = 1;
                }
            }
            Err(i) => {
                // 在 mid 列表的正确位置插入新 mid (索引为 (i+3), 因为第0,1,2位是元数据字段
                self.0.insert(i + 3, mid);
                self.0[2] += 1; // len是第2位
                let len = self.0[2] as usize;
                let i_index = len + i;
                let insert_index = self.0[i_index];
                self.0.insert(i_index, insert_index);
                let actual_insert_pos = (insert_index + 2) as usize;
                if let Some(last) = postfix {
                    // 先是标志位 0表示 mid 本身不存在，再是 postfix。
                    // 这里先插 postfix 再插 0 会得到 [0, last], 符合设计
                    self.0.insert(actual_insert_pos, last);
                    self.0.insert(actual_insert_pos, 0);
                    self.0[len..=i_index].iter_mut().for_each(|x| *x += 2);
                    let updated = &mut self.0[i_index + 1..len + len - 3];
                    for item in updated {
                        *item += 4; // 插入了 mid, 起始索引, 标志位 0, postfix 共 4 个元素, 所以后续起始索引 +4
                    }
                } else {
                    self.0.insert(actual_insert_pos, 1);
                    self.0[len..=i_index].iter_mut().for_each(|x| *x += 2);
                    let updated = &mut self.0[i_index + 1..len + len - 3];
                    for item in updated {
                        *item += 3;
                    }
                }
                self.0[len + len - 3] = self.0.len() as u32;
            }
        }
    }

    /// 查找是否包含指定的 mid 或 mid+postfix 组合
    /// postfix 为 None 时, 只检查 mid 本身是否存在
    /// postfix 为 Some 时, 在 mid 的 postfix 区间内二分查找
    fn find(&self, mid: u32, postfix: Option<u32>) -> bool {
        let len = self.0[2] as usize;
        let branches = &self.0[3..len];
        match branches.binary_search(&mid) {
            Ok(i) => {
                let i_index = len + i;
                let start = self.0[i_index] as usize;
                if let Some(last) = postfix {
                    let end = self.0[i_index + 1] as usize;
                    self.0[start + 1..end].binary_search(&last).is_ok()
                } else {
                    self.0[start] == 1
                }
            }
            Err(_) => false,
        }
    }
}
impl NgramTreeNode {
    pub fn new(prefix: u32) -> Self {
        let mut a = Vec::with_capacity(16);
        a.push(prefix);
        a.push(0); // 默认prefix的标志位为0
        a.push(3);
        a.push(4);
        NgramTreeNode(a)
    }
    pub fn add_prefix(&mut self) {
        self.0[1] = 1;
    }
    pub fn include_prefix(&self) -> bool {
        self.0[1] == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ngram_tree_node() {
        // 正常模式
        let mut node = NgramTreeNode::new(1);
        node.insert(1, Some(1));
        assert!(!node.find(1, None));
        assert!(!node.find(2, None));
        assert!(node.find(1, Some(1)));
        // 只插入mid
        node.insert(2, None);
        assert!(!node.find(1, Some(2)));
        assert!(!node.find(2, Some(1)));
        assert!(!node.find(1, None));
        assert!(node.find(2, None));
        assert!(!node.include_prefix());
        assert_eq!(node.0, vec![1u32, 0, 5, 1, 2, 8, 10, 11, 0, 1, 1]);
        node.add_prefix();
        assert!(node.include_prefix());
        assert_eq!(node.0, vec![1u32, 1, 5, 1, 2, 8, 10, 11, 0, 1, 1]);
    }
}
