use leetcode::s0001_two_sum::two_sum;

#[test]
fn test_case_1() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
}

#[test]
fn test_case_2() {
    assert_eq!(two_sum(vec![3, 2, 4], 6), [1, 2]);
}

#[test]
fn test_case_3() {
    assert_eq!(two_sum(vec![3, 3], 6), [0, 1]);
}
