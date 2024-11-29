pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut stones = stones;

    while stones.len() > 1 {
        let i = stones.len() - 1;
        stones.sort();

        if stones[i - 1] == stones[i] {
            stones = stones[..i - 1].to_vec();
        } else {
            let x = stones[i - 1];
            let y = stones[i];
            stones = stones[..i].to_vec();
            stones[i - 1] = y - x;
        }
    }

    if stones.len() == 0 {
        return 0;
    }

    stones[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(last_stone_weight(vec![1]), 1);
    }
}
