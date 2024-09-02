use std::collections::HashMap;

pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, usize> = HashMap::new();
    nums.iter().for_each(|n| {
        counter
            .entry(*n)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    let max_count: usize = *counter.values().max().unwrap();
    let nums_with_max_count: Vec<i32> = counter
        .iter()
        .filter(|(_, &count)| count == max_count)
        .map(|(num, _)| *num)
        .collect();

    let mut min_length: usize = nums.len();
    for num in &nums_with_max_count {
        let left: usize = nums.iter().position(|&n| n == *num).unwrap();
        let right: usize = nums.len() - nums.iter().rev().position(|&n| n == *num).unwrap();

        min_length = std::cmp::min(min_length, right - left);
    }

    min_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]), 6);
    }
}
