use std::collections::HashMap;

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut smallers_counter: Vec<i32> = vec![0; nums.len()];
    let mut smallers: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        if smallers.contains_key(&nums[i]) {
            smallers_counter[i] = smallers[&nums[i]];
            continue;
        }

        for j in 0..nums.len() {
            if nums[i] > nums[j] {
                smallers_counter[i] += 1;
            }
        }
        smallers.insert(nums[i], smallers_counter[i]);
    }

    smallers_counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            [4, 0, 1, 1, 3]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(smaller_numbers_than_current(vec![6, 5, 4, 8]), [2, 1, 0, 3]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(smaller_numbers_than_current(vec![7, 7, 7, 7]), [0, 0, 0, 0]);
    }
}
