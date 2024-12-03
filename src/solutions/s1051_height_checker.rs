pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut expected = heights.clone();
    expected.sort();

    let mut counter = 0;
    for i in 0..heights.len() {
        if heights[i] != expected[i] {
            counter += 1;
        }
    }

    counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(height_checker(vec![5, 1, 2, 3, 4]), 5);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(height_checker(vec![1, 2, 3, 4, 5]), 0);
    }
}
