pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
    let mut evens = nums.iter().step_by(2).copied().collect::<Vec<i32>>();
    let mut odds = nums
        .iter()
        .skip(1)
        .step_by(2)
        .copied()
        .collect::<Vec<i32>>();

    evens.sort_unstable();
    odds.sort_unstable_by(|a, b| b.cmp(a));

    (0..nums.len())
        .map(|i| {
            if i % 2 == 0 {
                evens[i / 2]
            } else {
                odds[i / 2]
            }
        })
        .collect()
}
