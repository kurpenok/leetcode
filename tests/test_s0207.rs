use leetcode::s0207_course_schedule::can_finish;

#[test]
fn test_case_1() {
    assert_eq!(can_finish(2, vec![vec![1, 0]]), true);
}

#[test]
fn test_case_2() {
    assert_eq!(can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
}
