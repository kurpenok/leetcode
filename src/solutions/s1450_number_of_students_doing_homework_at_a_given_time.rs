pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    start_time
        .iter()
        .zip(end_time.iter())
        .filter(|(&start, &end)| start <= query_time && query_time <= end)
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(busy_student(vec![4], vec![4], 4), 1);
    }
}
