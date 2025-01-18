pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
    let mut tax = 0.0;
    let mut remain = income;
    let mut prev_bound = 0;

    for pair in &brackets {
        if remain == 0 {
            break;
        }

        let (upper_bound, percent) = (pair[0], pair[1]);
        let sum = remain.min(upper_bound - prev_bound);
        tax += (sum * percent) as f64 / 100.0;
        remain -= sum;
        prev_bound = upper_bound;
    }

    tax
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10),
            2.65,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2),
            0.25,
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(calculate_tax(vec![vec![2, 50]], 0), 0.0);
    }
}
