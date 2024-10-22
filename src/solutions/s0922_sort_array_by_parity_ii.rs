pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
    let mut evens = Vec::new();
    let mut odds = Vec::new();
    for num in &nums {
        if num % 2 == 0 {
            evens.push(*num);
        } else {
            odds.push(*num);
        }
    }

    let mut sorted_nums = Vec::new();
    let mut i = 0;
    let mut j = 0;
    for k in 0..nums.len() {
        if k % 2 == 0 {
            sorted_nums.push(evens[i]);
            i += 1;
        } else {
            sorted_nums.push(odds[j]);
            j += 1;
        }
    }

    sorted_nums
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(sort_array_by_parity_ii(vec![4, 2, 5, 7]), [4, 5, 2, 7]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sort_array_by_parity_ii(vec![2, 3]), [2, 3]);
    }
}
