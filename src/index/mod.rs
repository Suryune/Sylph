pub mod ngram_tries;
pub mod term_trie;

pub trait TreeNode {
    fn insert(&mut self, mid: u32, postfix: Option<u32>);
    fn find(&self, mid: u32, postfix: Option<u32>) -> bool;
}
