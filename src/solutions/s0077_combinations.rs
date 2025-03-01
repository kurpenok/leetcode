fn helper(start: i32, k: i32, n: i32, temp: &mut Vec<i32>, combinations: &mut Vec<Vec<i32>>) {
    if temp.len() == k as usize {
        combinations.push(temp.clone());
        return;
    }

    for i in start..=n {
        temp.push(i);
        helper(i + 1, k, n, temp, combinations);
        temp.pop();
    }
}

pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut combinations: Vec<Vec<i32>> = Vec::new();
    helper(1, k, n, &mut Vec::new(), &mut combinations);
    combinations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            combine(4, 2),
            [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(combine(1, 1), [[1]]);
    }
}
