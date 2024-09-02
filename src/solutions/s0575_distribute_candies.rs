use std::collections::HashSet;

pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let count_of_unique_candies: HashSet<i32> = candy_type.clone().into_iter().collect();

    if count_of_unique_candies.len() > candy_type.len() / 2 {
        return (candy_type.len() / 2) as i32;
    }

    count_of_unique_candies.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(distribute_candies(vec![1, 1, 2, 3]), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
