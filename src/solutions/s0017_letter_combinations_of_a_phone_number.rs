fn helper(
    combination: String,
    next_digits: &str,
    letters_by_digit: &[&str],
    combinations: &mut Vec<String>,
) {
    if next_digits.is_empty() {
        combinations.push(combination);
        return;
    }

    let digit = next_digits.chars().next().unwrap().to_digit(10).unwrap() as usize;
    for letter in letters_by_digit[digit].chars() {
        let mut new_combinations = combination.clone();
        new_combinations.push(letter);
        helper(
            new_combinations,
            &next_digits[1..],
            letters_by_digit,
            combinations,
        );
    }
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let mut combinations: Vec<String> = Vec::new();

    let letters_by_digit = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];

    helper(String::new(), &digits, &letters_by_digit, &mut combinations);

    combinations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            letter_combinations("23".to_string()),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(letter_combinations("".to_string()), Vec::<String>::new());
    }

    #[test]
    fn test_case_3() {
        assert_eq!(letter_combinations("2".to_string()), ["a", "b", "c"]);
    }
}
