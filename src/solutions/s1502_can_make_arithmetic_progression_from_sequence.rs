pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
    let mut arr = arr;
    arr.sort();

    for w in arr.windows(3) {
        if w[2] - w[1] != w[1] - w[0] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(can_make_arithmetic_progression(vec![3, 5, 1]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(can_make_arithmetic_progression(vec![1, 2, 4]), false);
    }
}
