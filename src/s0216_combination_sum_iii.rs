fn helper(
    start: i32,
    k: i32,
    n: i32,
    current_sum: i32,
    combination: &mut Vec<i32>,
    combinations: &mut Vec<Vec<i32>>,
) {
    if combination.len() as i32 == k {
        if current_sum == n {
            combinations.push(combination.clone());
        }
        return;
    }

    for i in start..10 {
        if current_sum + i > n {
            break;
        }
        combination.push(i);
        helper(i + 1, k, n, current_sum + i, combination, combinations);
        combination.pop();
    }
}

pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut combinations: Vec<Vec<i32>> = Vec::new();
    let mut combination: Vec<i32> = Vec::new();

    helper(1, k, n, 0, &mut combination, &mut combinations);

    combinations
}
