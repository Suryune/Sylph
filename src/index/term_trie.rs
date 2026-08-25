use crate::index::TreeNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermTrie {
    roots: Vec<TermTrieNode>,
}

/// 扁平的树状节点,存储某个 prefix 下所有 mid 及其 postfix
/// 内部数组布局:
/// [0]: 分支数+1 (即 mid 数量+1)
/// [1..len]: 按升序排列的 mid
/// [len..len+len-1]: 每个 mid 对应 postfix 区间的起始索引
/// [len + len - 1]: 数组总长度
/// 剩余部分: 按区间连续存放的 postfix 值
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermTrieNode(pub Vec<u32>);

impl TreeNode for TermTrieNode {
    /// 插入一个 (mid, postfix) 对, postfix可以为None
    /// 仅在任意一个值不存在于数组中时插入
    /// 保持内部数组有序
    fn insert(&mut self, mid: u32, postfix: Option<u32>) {
        let len = self.0[0] as usize; // mid的数量+1,也就是存储的索引的位置
        let branches = &self.0[1..len]; // len存储的数据相当于第一个 mid 范围的起始索引的索引, 则前面一个自然就是最后一个mid
        match branches.binary_search(&mid) {
            Ok(i) => {
                if let Some(last) = postfix {
                    let i_index = len + i; // 第i个起始索引在数组中的位置
                    let next_mid_index = i_index + 1; // 下一个 mid 的起始索引位置(可能指向总长度字段)
                    let start = self.0[i_index] as usize; // 当前 mid 的 postfix 区间起点
                    let end = self.0[next_mid_index] as usize; // 下一个 mid 的postfix区间起点, 也就是当前 mid 的 postfix 区间终点(不包含)
                    match &self.0[start..end].binary_search(&last) {
                        Ok(_) => (), // postfix已经存在, 无操作
                        Err(insert_index) => {
                            self.0.insert(start + insert_index, last);

                            // 插入导致后续所有元素往后移一位, 需要更新之后所有 mid 的起始索引
                            // 受影响范围: 从下一个 mid 的起始索引开始, 到起始索引列表结束 (总长度字段单独更新)
                            let updated = &mut self.0[next_mid_index..len + len - 1];
                            for item in updated {
                                *item += 1;
                            }
                            self.0[len + len - 1] = self.0.len() as u32; //更新总长度字段
                        }
                    }
                }
            }
            Err(i) => {
                // 在 mid 列表的正确位置插入新 mid (索引为 (i+1), 因为第 0 位是 len 字段
                self.0.insert(i + 1, mid);
                // 分支数量加1, 更新len
                self.0[0] += 1;
                let len = self.0[0] as usize;
                let i_index = len + i;
                let insert_index = self.0[i_index]; // 新 mid 对应的起始索引位置
                                                    // 取原位置(即下一个mid)的值作为新 mid 的起始索引
                                                    // 新 mid 的起始索引等于它, 而因为取不到区间中点, 形成空区间
                                                    // 用的是还没有插入新 mid 时的索引, 需要更新, 后面统一更新(如果要用就要+2)
                self.0.insert(i_index, insert_index);
                if let Some(last) = postfix {
                    self.0.insert((insert_index + 2) as usize, last); // 转换成实际的索引
                                                                      // 当前 mid 及其之前的起始索引: 因为插入了 mid 和起始索引(共2个元素), 所以需要 +2
                                                                      // 当前 mid 之后的起始索引: 除了上述 +2 外, 还要加上插入 postfix 造成的 +1, 因此总共 +3
                    self.0[len..=i_index].iter_mut().for_each(|x| *x += 2);
                    let updated = &mut self.0[i_index + 1..len + len - 1];
                    for item in updated {
                        *item += 3;
                    }
                } else {
                    let updated = &mut self.0[len..len + len - 1];
                    for item in updated {
                        *item += 2;
                    }
                }
                self.0[len + len - 1] = self.0.len() as u32;
            }
        }
    }
    /// 查看是否含有这个元素
    fn find(&self, mid: u32, postfix: Option<u32>) -> bool {
        let len = self.0[0] as usize;
        let branches = &self.0[1..len];
        match branches.binary_search(&mid) {
            Ok(i) => {
                if let Some(last) = postfix {
                    let i_index = len + i;
                    let start = self.0[i_index] as usize;
                    let end = self.0[i_index + 1] as usize;
                    self.0[start..end].binary_search(&last).is_ok()
                } else {
                    true
                }
            }
            Err(_) => false,
        }
    }
}

impl TermTrieNode {
    pub fn new() -> Self {
        let mut a = Vec::with_capacity(16);
        // [0]存放 (分支数+1), 即 mid 的数量+1
        a.push(1);
        // 初始时没有分支, 总长度字段暂存为2(占位), 实际长度在插入时更新
        a.push(2);
        TermTrieNode(a)
    }
    pub fn is_empty(&self) -> bool {
        // 空节点内部数组只有两个元素: [分支数+1(=1), 总长度(=2)]
        matches!(self.0.len(), 2)
    }
}

impl TermTrie {
    pub fn new() -> Self {
        TermTrie {
            roots: Vec::with_capacity(1000),
        }
    }
    pub fn ensure(&mut self, prefix: usize) {
        if self.roots.len() < prefix {
            self.roots.resize(prefix, TermTrieNode::new());
        }
    }
    pub fn insert(&mut self, prefix: u32, mid: Option<u32>, postfix: Option<u32>) {
        self.ensure(prefix as usize);
        if let Some(mid_value) = mid {
            self.roots[prefix as usize].insert(mid_value, postfix);
        }
    }
    pub fn find(&self, prefix: u32, mid: Option<u32>, postfix: Option<u32>) -> bool {
        if self.roots.len() < prefix as usize {
            return false;
        }
        if let Some(mid_value) = mid {
            self.roots[prefix as usize].find(mid_value, postfix)
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn term_trie_node() {
        // 只插入mid
        let mut node = TermTrieNode::new();
        assert!(node.is_empty());
        node.insert(1, None);
        assert!(node.find(1, None));
        assert!(!node.find(2, None));
        assert!(!node.find(1, Some(1)));
        // 正常模式,插入mid和postfix
        node.insert(2, Some(1));
        assert!(!node.find(1, Some(1)));
        assert!(node.find(2, Some(1)));
        assert!(node.find(2, None));
        assert_eq!(node.0, vec![3u32, 1, 2, 6, 6, 7, 1]);
    }
}
