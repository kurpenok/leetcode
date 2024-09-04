use std::collections::HashSet;

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let jewels: HashSet<char> = HashSet::from_iter(jewels.chars());
    let stones: Vec<char> = stones.chars().collect();

    let mut jewels_count: i32 = 0;
    stones.iter().for_each(|stone| {
        if jewels.contains(stone) {
            jewels_count += 1;
        }
    });

    jewels_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
    }
}
