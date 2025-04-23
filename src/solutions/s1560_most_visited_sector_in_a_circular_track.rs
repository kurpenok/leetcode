pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
    let &start: &i32 = rounds.first().unwrap();
    let &end: &i32 = rounds.last().unwrap();

    match end >= start {
        true => (start..=end).collect(),
        false => (1..=end).chain(start..=n).collect(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(most_visited(4, vec![1, 3, 1, 2]), [1, 2]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(most_visited(2, vec![1, 3, 5, 7]), [1, 2, 3, 4, 5, 6, 7]);
    }
}
