use std::collections::HashMap;

pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut arr = arr;
    arr.sort();

    let mut diffs: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
    for i in 1..arr.len() {
        let diff = arr[i] - arr[i - 1];
        let pair = vec![arr[i - 1], arr[i]];
        diffs
            .entry(diff)
            .and_modify(|pairs| pairs.push(pair.clone()))
            .or_insert(vec![pair.clone()]);
    }

    diffs[diffs.keys().min().unwrap()].clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            minimum_abs_difference(vec![4, 2, 1, 3]),
            [[1, 2], [2, 3], [3, 4]],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(minimum_abs_difference(vec![1, 3, 6, 10, 15]), [[1, 3]]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
            [[-14, -10], [19, 23], [23, 27]],
        );
    }
}
