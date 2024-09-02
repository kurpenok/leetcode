use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let roman_digits: Vec<char> = s.chars().collect();

    let codes: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut n: i32 = codes[&roman_digits[0]];

    for i in 1..s.len() {
        if codes[&roman_digits[i - 1]] < codes[&roman_digits[i]] {
            n += codes[&roman_digits[i]] - (2 * codes[&roman_digits[i - 1]]);
        } else {
            n += codes[&roman_digits[i]];
        }
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
