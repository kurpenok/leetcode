pub fn average(salary: Vec<i32>) -> f64 {
    (salary.iter().sum::<i32>() as f64
        - *salary.iter().min().unwrap() as f64
        - *salary.iter().max().unwrap() as f64)
        / (salary.len() - 2) as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(average(vec![4000, 3000, 1000, 2000]), 2500.0);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(average(vec![1000, 2000, 3000]), 2000.0);
    }
}
