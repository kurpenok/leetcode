pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    arr.iter()
        .enumerate()
        .map(|(i, _)| {
            if i == arr.len() - 1 {
                -1
            } else {
                *arr[i + 1..].iter().max().unwrap()
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            replace_elements(vec![17, 18, 5, 4, 6, 1]),
            [18, 6, 6, 6, 1, -1],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(replace_elements(vec![400]), [-1]);
    }
}
