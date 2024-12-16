use std::collections::HashMap;

pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut sum_of_pairs = 0;
    let mut pairs: HashMap<(i32, i32), usize> = HashMap::new();

    for domino in dominoes {
        let mut key = domino;
        key.sort();
        let key = (key[0], key[1]);

        if pairs.contains_key(&key) {
            let count = *pairs.get(&key).unwrap();
            sum_of_pairs += count;
            pairs.insert(key, count + 1);
        } else {
            pairs.insert(key, 1);
        }
    }

    sum_of_pairs as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
            1,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            num_equiv_domino_pairs(vec![
                vec![1, 2],
                vec![1, 2],
                vec![1, 1],
                vec![1, 2],
                vec![2, 2],
            ]),
            3,
        );
    }
}
