/// 具体说明见 [tokenizer.md](../doc/tokenizer.md)
use crate::index::{ngram_tries::NgramTreeNode, TreeNode};
use ahash::AHashMap;
use std::mem::take;

/// 将一段文本按标点拆分为句子,
/// 再将每个句子拆分成单词或单个汉字.
pub fn split_into_sentences(text: &str) -> Vec<Vec<String>> {
    let mut chunks: Vec<Vec<String>> = Vec::new();
    let mut chunk: Vec<String> = Vec::new();
    let mut word = String::new();
    for a in text.chars() {
        if a == ' ' {
            if !word.is_empty() {
                chunk.push(take(&mut word));
            }
            continue;
        }
        if !a.is_alphanumeric() {
            if a == '-' {
                word.push('-');
            } else {
                if !word.is_empty() {
                    chunk.push(take(&mut word));
                }
                if chunk.is_empty() {
                    continue;
                }
                chunks.push(take(&mut chunk));
            }
            continue;
        }
        if a.is_ascii() {
            word.push(a);
        } else {
            if !word.is_empty() {
                chunk.push(take(&mut word));
            }
            chunk.push(a.to_string());
        }
    }
    if !word.is_empty() {
        chunk.push(take(&mut word));
    }
    if !chunk.is_empty() {
        chunks.push(chunk);
    }
    chunks
}

/// 维护一个全局哈希表(AHashMap<String, u32>),
/// 将 split_into_sentences 产出的每个单词映射为一个唯一的整数ID.
/// 若单词不存在则为其分配新 ID(当前词表大小) 并加入哈希表.
pub fn assign_ids(
    id_map: &mut AHashMap<String, u32>,
    len: &mut u32,
    context: Vec<Vec<String>>,
) -> Vec<Vec<u32>> {
    let mut id_list: Vec<Vec<u32>> = Vec::new();
    let mut ids: Vec<u32> = Vec::new();
    for line in context {
        for word in line {
            let next_id = *len;
            let id = *id_map.entry(word).or_insert_with(|| {
                *len += 1;
                next_id
            });
            ids.push(id);
        }
        id_list.push(take(&mut ids));
    }
    id_list
}

/// 接受一个全局哈希表(AHashMap<String, u32>),
/// 将 split_into_sentences 产出的每个单词映射为一个唯一的整数ID.
/// 若单词不存在则跳过, 如果一行中所有单词都不在词表中, 则跳过该行, 不会在结果中产生空列表.
pub fn lookup_ids(id_map: &AHashMap<String, u32>, context: Vec<Vec<String>>) -> Vec<Vec<u32>> {
    let mut id_list: Vec<Vec<u32>> = Vec::new();
    let mut ids: Vec<u32> = Vec::new();
    for line in context {
        for word in line {
            if let Some(&id) = id_map.get(&word) {
                ids.push(id);
            }
        }
        if !ids.is_empty() {
            id_list.push(take(&mut ids));
        }
    }
    id_list
}

/// 接受 assign_ids 输出的 ID 列表, 生产3-gram,
/// 如果长度不够可能只生成1gram或2gram.
pub fn generate_ngrams(context: Vec<Vec<u32>>) -> Vec<NgramTreeNode> {
    let mut list: Vec<NgramTreeNode> = Vec::with_capacity(context.len());

    let mut id_map: AHashMap<u32, usize> = AHashMap::new();
    for line in context {
        if line.len() == 0 {
            continue;
        }
        if line.len() > 2 {
            for window in line.windows(3) {
                let index: usize = match id_map.get(&window[0]) {
                    Some(&i) => i,
                    None => {
                        let i = list.len();
                        id_map.insert(window[0], i);
                        list.push(NgramTreeNode::new(window[0]));
                        i
                    }
                };
                list[index].insert(window[1], Some(window[2]));
            }
        } else {
            let index: usize = match id_map.get(&line[0]) {
                Some(&i) => i,
                None => {
                    let i = list.len();
                    id_map.insert(line[0], i);
                    list.push(NgramTreeNode::new(line[0]));
                    i
                }
            };
            if line.len() == 2 {
                list[index].insert(line[1], None);
            } else {
                list[index].add_prefix();
            }
        }
    }
    list
}
