use std::collections::HashMap;

pub fn find_lucky(arr: Vec<i32>) -> i32 {
    arr.iter()
        .fold(HashMap::new(), |mut map: HashMap<i32, i32>, &x| {
            *map.entry(x).or_default() += 1;
            map
        })
        .iter()
        .filter(|(key, count)| key == count)
        .max_by_key(|(key, _)| *key)
        .map_or(-1, |(&k, _)| k)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_lucky(vec![2, 2, 3, 4]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_lucky(vec![2, 2, 2, 3, 3]), -1);
    }
}
