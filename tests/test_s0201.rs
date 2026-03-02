use leetcode::s0201_bitwise_and_of_numbers_range::range_bitwise_and;

#[test]
fn test_case_1() {
    assert_eq!(range_bitwise_and(5, 7), 4);
}

#[test]
fn test_case_2() {
    assert_eq!(range_bitwise_and(0, 0), 0);
}

#[test]
fn test_case_3() {
    assert_eq!(range_bitwise_and(1, 2147483647), 0);
}
