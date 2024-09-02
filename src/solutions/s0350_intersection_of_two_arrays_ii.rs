use std::collections::HashMap;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut set1: HashMap<i32, i32> = HashMap::new();
    for n in nums1 {
        set1.entry(n).and_modify(|n| *n += 1).or_insert(1);
    }

    let mut set2: HashMap<i32, i32> = HashMap::new();
    for n in nums2 {
        set2.entry(n).and_modify(|n| *n += 1).or_insert(1);
    }

    for (n, count) in &set1 {
        if set2.contains_key(n) {
            for _ in 0..(std::cmp::min(*count, set2[n])) {
                result.push(*n);
            }
        }
    }

    result
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_case_1() {
//        assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
//    }
//
//    #[test]
//    fn test_case_2() {
//        assert_eq!(intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
//    }
//}
