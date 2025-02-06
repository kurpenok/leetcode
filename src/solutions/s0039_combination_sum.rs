use std::collections::HashSet;

fn helper(
    combination: &mut Vec<i32>,
    combinations: &mut HashSet<Vec<i32>>,
    candidates: &Vec<i32>,
    target: i32,
) {
    let sum = combination.iter().sum::<i32>();

    if sum == target {
        let mut combination = combination.clone();
        combination.sort();
        combinations.insert(combination);
        return;
    }

    if sum > target {
        return;
    }

    for candidate in candidates {
        if sum + candidate <= target {
            combination.push(*candidate);
            helper(combination, combinations, candidates, target);
            combination.pop();
        }
    }
}
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut combinations: HashSet<Vec<i32>> = HashSet::new();
    helper(&mut vec![], &mut combinations, &candidates, target);

    let mut combinations: Vec<Vec<i32>> = combinations.into_iter().collect();
    combinations.sort();
    combinations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }

    #[test]
    fn test_case_3() {
        assert!(combination_sum(vec![2], 1).is_empty());
    }
}
