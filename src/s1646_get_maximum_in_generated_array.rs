pub fn get_maximum_generated(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let n = n as usize;

    let mut nums = vec![0; n + 1];
    nums[0] = 0;
    nums[1] = 1;

    let mut max = 1;

    for i in 1..=n / 2 {
        if 2 * i <= n {
            nums[2 * i] = nums[i];
            max = max.max(nums[2 * i]);
        }

        if 2 * i + 1 <= n {
            nums[2 * i + 1] = nums[i] + nums[i + 1];
            max = max.max(nums[2 * i + 1]);
        }
    }

    max
}
