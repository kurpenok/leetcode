pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n: usize = nums.len();
    let k: usize = k as usize % n;

    let result: Vec<i32> = nums.iter().cloned().collect();
    for i in 0..n {
        nums[(i + k) % n] = result[i];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];

        rotate(&mut nums, 3);

        assert_eq!(nums, [5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_case_2() {
        let mut nums: Vec<i32> = vec![-1, -100, 3, 99];

        rotate(&mut nums, 2);

        assert_eq!(nums, [3, 99, -1, -100]);
    }
}
