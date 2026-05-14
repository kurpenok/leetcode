#[cfg(test)]
mod test_s1700 {
    use leetcode::s1700_number_of_students_unable_to_eat_lunch::count_students;

    #[test]
    fn test_case_1() {
        assert_eq!(count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]), 0);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
