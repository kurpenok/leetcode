pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
    std::cmp::min(
        position.iter().filter(|&p| p % 2 == 0).count(),
        position.iter().filter(|&p| p % 2 != 0).count(),
    ) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(min_cost_to_move_chips(vec![1, 2, 3]), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(min_cost_to_move_chips(vec![1, 1000000000]), 1);
    }
}
