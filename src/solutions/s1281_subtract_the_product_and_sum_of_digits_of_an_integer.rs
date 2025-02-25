pub fn subtract_product_and_sum(n: i32) -> i32 {
    let digits: Vec<i32> = n
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect();

    let digits_product: i32 = digits.iter().product();
    let digits_sum: i32 = digits.iter().sum();

    digits_product - digits_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(subtract_product_and_sum(234), 15);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(subtract_product_and_sum(4421), 21);
    }
}
