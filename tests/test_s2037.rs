#[cfg(test)]
mod test_s2037 {
    use leetcode::s2037_minimum_number_of_moves_to_seat_everyone::min_moves_to_seat;

    #[test]
    fn test_case_1() {
        assert_eq!(min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]), 7);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]), 4);
    }
}
