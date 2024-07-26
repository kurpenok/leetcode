use std::collections::HashMap;

pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut numbers: HashMap<i32, i32> = HashMap::new();
    let mut max_length = 0;

    for n in nums {
        let k = *numbers
            .entry(n)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);

        if let Some(&v) = &numbers.get(&(n - 1)) {
            max_length = max_length.max(v + k);
        }

        if let Some(&v) = &numbers.get(&(n + 1)) {
            max_length = max_length.max(v + k);
        }
    }

    max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_lhs(vec![1, 2, 3, 4]), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_lhs(vec![1, 1, 1, 1]), 0);
    }
}
