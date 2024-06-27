pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            result.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            result.push("Fizz".to_string());
        } else if i % 5 == 0 {
            result.push("Buzz".to_string());
        } else {
            result.push(i.to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(fizz_buzz(3), vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
