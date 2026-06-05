pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
    nums.iter()
        .enumerate()
        .filter(|&(_, &v)| v == target)
        .map(|(i, _)| (i as i32 - start).abs())
        .min()
        .unwrap()
}
