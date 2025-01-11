use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    arr.iter()
        .fold(HashMap::new(), |mut map, &x| {
            *map.entry(x).or_insert(0) += 1;
            map
        })
        .values()
        .cloned()
        .collect::<HashSet<usize>>()
        .len()
        == arr.iter().cloned().collect::<HashSet<i32>>().len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(unique_occurrences(vec![1, 2]), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
            true,
        );
    }
}
