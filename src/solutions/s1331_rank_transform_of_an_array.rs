use std::collections::HashMap;

pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut unique_elements: Vec<i32> = arr.clone();
    unique_elements.sort();
    unique_elements.dedup();

    let mut rank_map: HashMap<i32, i32> = HashMap::new();
    for (rank, &value) in unique_elements.iter().enumerate() {
        rank_map.insert(value, rank as i32 + 1);
    }

    arr.iter()
        .map(|&value| *rank_map.get(&value).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(array_rank_transform(vec![40, 10, 20, 30]), [4, 1, 2, 3]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(array_rank_transform(vec![100, 100, 100]), [1, 1, 1]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            [5, 3, 4, 2, 8, 6, 7, 1, 3],
        );
    }
}
