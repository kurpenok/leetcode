use std::collections::HashMap;

pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    arr.iter().fold(HashMap::new(), |mut map, n| {
        let _ = map.entry(*n).and_modify(|count| *count += 1).or_insert(1);
        map
    }) == target.iter().fold(HashMap::new(), |mut map, n| {
        let _ = map.entry(*n).and_modify(|count| *count += 1).or_insert(1);
        map
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(can_be_equal(vec![7], vec![7]), true);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(can_be_equal(vec![3, 7, 9], vec![3, 7, 11]), false);
    }
}
