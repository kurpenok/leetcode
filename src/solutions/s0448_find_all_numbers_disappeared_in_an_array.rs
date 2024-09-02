use std::collections::HashSet;

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut stock: HashSet<i32> = HashSet::new();
    for num in &nums {
        if !stock.contains(num) {
            stock.insert(*num);
        }
    }

    for i in 1..=nums.len() {
        if !stock.contains(&(i as i32)) {
            result.push(i as i32);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_disappeared_numbers(vec![1, 1]), vec![2]);
    }
}
