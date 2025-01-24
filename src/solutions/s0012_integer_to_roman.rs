pub fn int_to_roman(num: i32) -> String {
    let mut number = num;

    let mut roman_numeral = String::new();

    let symbols = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    for &(symbol, value) in &symbols {
        while number >= value {
            roman_numeral.push_str(symbol);
            number -= value;
        }
    }

    roman_numeral
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(int_to_roman(58), "LVIII");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }
}
