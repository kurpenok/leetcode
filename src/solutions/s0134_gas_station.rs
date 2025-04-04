pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total_gas: i32 = 0;
    let mut total_cost: i32 = 0;
    let mut current_gas: i32 = 0;
    let mut start_index: usize = 0;

    for i in 0..gas.len() {
        total_gas += gas[i];
        total_cost += cost[i];
        current_gas += gas[i] - cost[i];

        if current_gas < 0 {
            start_index = i + 1;
            current_gas = 0;
        }
    }

    if total_gas >= total_cost {
        start_index as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
    }
}
