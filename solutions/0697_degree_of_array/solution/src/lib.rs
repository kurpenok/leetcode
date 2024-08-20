use std::collections::HashMap;

pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, usize> = HashMap::new();

    for n in &nums {
        counter
            .entry(*n)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let max_value: usize = *counter.values().max().unwrap();
    let keys_with_max_value: Vec<i32> = counter
        .iter()
        .filter(|(_, &v)| v == max_value)
        .map(|(k, _)| *k)
        .collect();

    let mut min_length: usize = nums.len();
    for key in &keys_with_max_value {
        let left: usize = nums.iter().position(|&e| e == *key).unwrap();
        let right: usize = nums.len() - nums.iter().rev().position(|&e| e == *key).unwrap();

        println!("{} {} {}", key, left, right);

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
