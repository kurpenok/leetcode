use leetcode::s0210_course_schedule_ii::find_order;

#[test]
fn test_case_1() {
    assert_eq!(find_order(2, vec![vec![1, 0]]), [0, 1]);
}

#[test]
fn test_case_2() {
    assert_eq!(
        find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
        [0, 1, 2, 3]
    );
}

#[test]
fn test_case_3() {
    assert_eq!(find_order(1, vec![]), [0]);
}
