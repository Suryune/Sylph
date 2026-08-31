use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct GapVec {
    pub first: u64,
    pub len: u64,
    pub gaps: Column,
    pub patches: Vec<u64>, // 存储的是绝对文档ID
    pub patch_positions: Vec<u64>,
}

macro_rules! gen_compress_branch {
    (
        $type:ty,           // 如 u8
        $threshold:expr,    // 如 u8_max
        $variant:ident,     // 枚举变体名, 如 Uint8
        $list:expr,         // &Vec<u64>
        $len:expr,          // len 变量
        $gaps_cap:expr,     // 容量 = len - 异常数量 (预先计算)
        $patches:expr,      // &mut Vec<u64>
        $patch_positions:expr, // &mut Vec<u64>
        $patches_skipped:expr  // &mut u64
    ) => {{
        let mut gap_list = Vec::with_capacity($gaps_cap as usize);
        for i in 1..$len {
            let delta = $list[i] - $list[i - 1];
            if delta > $threshold {
                $patch_positions.push(i as u64 - *$patches_skipped);
                $patches.push($list[i]);
                *$patches_skipped += 1;
            } else {
                gap_list.push(delta as $type);
            }
        }
        Column::$variant(gap_list)
    }};
}
macro_rules! gen_decode_branch {
    ($variant:ident, $self:expr, $gaps:expr) => {{
        let mut list: Vec<u64> = Vec::with_capacity($self.len as usize);
        let mut item = $self.first;
        list.push(item);
        for arm_idx in 1..$self.patch_positions.len() {
            let prev_idx = arm_idx - 1;
            for b in $self.patch_positions[prev_idx]..$self.patch_positions[arm_idx] {
                item += $gaps[b as usize] as u64;
                list.push(item);
            }
            list.push($self.patches[prev_idx]);
            item = $self.patches[prev_idx];
        }
        for i in *$self.patch_positions.last().unwrap() as usize..$gaps.len() {
            item += $gaps[i] as u64;
            list.push(item);
        }
        list
    }};
}
impl GapVec {
    /// 转换成增量列表,需要长度>=2
    pub fn new(list: Vec<u64>) -> Self {
        let first = list[0];
        let len = list.len();
        // 获取升序的增量的列表
        let mut deltas: Vec<u64> = list.windows(2).map(|w| w[1] - w[0]).collect();
        deltas.sort_unstable();

        // 判断哪种阈值更合算
        let u8_max = u8::MAX as u64;
        let u16_max = u16::MAX as u64;
        let u32_max = u32::MAX as u64;

        let mut min_pair: (Threshold, (u64, u64)); // (使用的类型, (patch的数量, 占用的大小))
        min_pair = (Threshold::Uint8, estimate_bits(u8_max, 8, &deltas));
        let u16_size = estimate_bits(u16_max, 16, &deltas);
        if u16_size.1 <= min_pair.1.1 {
            min_pair = (Threshold::Uint16, u16_size);
        }
        let u32_size = estimate_bits(u32_max, 32, &deltas);
        if u32_size.1 <= min_pair.1.1 {
            min_pair = (Threshold::Uint32, u32_size);
        }

        // 生成对应的增量列表
        let mut patches: Vec<u64> = Vec::with_capacity(min_pair.1.0 as usize);
        let mut patch_positions: Vec<u64> = Vec::with_capacity(min_pair.1.0 as usize + 1);
        patch_positions.push(0); // 表示范围的起始
        let mut patches_skipped: u64 = 1; // 表示已经处理的异常值的数量+1, 用于修正索引(因为异常的和第0位没有被添加到gaps,一些索引会往前移)

        let gaps = match min_pair.0 {
            Threshold::Uint8 => gen_compress_branch!(
                u8,
                u8_max,
                Uint8,
                &list,
                len,
                len - min_pair.1.0 as usize,
                &mut patches,
                &mut patch_positions,
                &mut patches_skipped
            ),
            Threshold::Uint16 => gen_compress_branch!(
                u16,
                u16_max,
                Uint16,
                &list,
                len,
                len - min_pair.1.0 as usize,
                &mut patches,
                &mut patch_positions,
                &mut patches_skipped
            ),
            Threshold::Uint32 => gen_compress_branch!(
                u32,
                u32_max,
                Uint32,
                &list,
                len,
                len - min_pair.1.0 as usize,
                &mut patches,
                &mut patch_positions,
                &mut patches_skipped
            ),
        };
        GapVec {
            first,
            len: len as u64,
            gaps,
            patches,
            patch_positions,
        }
    }
    pub fn into_list(self) -> Vec<u64> {
        match self.gaps {
            Column::Uint8(gaps) => gen_decode_branch!(Uint8, self, gaps),
            Column::Uint16(gaps) => gen_decode_branch!(Uint16, self, gaps),
            Column::Uint32(gaps) => gen_decode_branch!(Uint32, self, gaps),
        }
    }
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum Column {
    Uint8(Vec<u8>),
    Uint16(Vec<u16>),
    Uint32(Vec<u32>),
}
pub enum Threshold {
    Uint8,
    Uint16,
    Uint32,
}
fn estimate_bits(threshold: u64, size: u64, list: &[u64]) -> (u64, u64) {
    let index = list.binary_search(&threshold).unwrap_or_else(|e| e) as u64;
    let mut len: u64 = index * size; // 正常差值总比特数
    let patch_len = list.len() as u64 - index;
    len += patch_len * 128; // 异常值额外开销: 每个异常 2 个 u64 即 128 比特
    (patch_len, len)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gap_vec() {
        let list = vec![0u64, 1, 3, 7, 262, 517, 9999, 19999, 199999, 202608, 260831];
        let gap_list = GapVec::new(list.clone()).gaps;
        let gap_vec = GapVec::new(list.clone());
        assert!(matches!(gap_list, Column::Uint16(_)));
        let list_2 = gap_vec.into_list();
        assert_eq!(list, list_2);
    }
}
