impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        n.count_ones()
    }
}
