fn helper(
    combination: &mut Vec<i32>,
    remain: i32,
    start: usize,
    candidates: &[i32],
    combinations: &mut Vec<Vec<i32>>,
) {
    if remain == 0 {
        combinations.push(combination.clone());
        return;
    }

    for i in start..candidates.len() {
        if i > start && candidates[i] == candidates[i - 1] {
            continue;
        }

        if candidates[i] > remain {
            break;
        }

        combination.push(candidates[i]);
        helper(
            combination,
            remain - candidates[i],
            i + 1,
            candidates,
            combinations,
        );
        combination.pop();
    }
}

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut combinations: Vec<Vec<i32>> = Vec::new();

    let mut candidates = candidates;
    candidates.sort();

    helper(&mut vec![], target, 0, &candidates, &mut combinations);

    combinations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
