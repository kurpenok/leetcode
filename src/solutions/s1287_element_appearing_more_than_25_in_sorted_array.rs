use std::collections::HashMap;

pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    arr.iter()
        .fold(HashMap::new(), |mut cntr, &n| {
            let _ = *cntr.entry(n).and_modify(|count| *count += 1).or_insert(1);
            cntr
        })
        .iter()
        .max_by_key(|&(_, value)| value)
        .map(|(&key, _)| key)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_special_integer(vec![1, 1]), 1);
    }
}
