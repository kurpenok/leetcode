impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut mnums = nums.to_vec();
        mnums.sort();

        for i in (0..mnums.len() - 1).step_by(2) {
            if mnums[i] != mnums[i + 1] {
                return mnums[i];
            }
        }

        return mnums[mnums.len() - 1];
    }
}
