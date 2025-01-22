pub fn my_atoi(s: String) -> i32 {
    let s = s.trim();

    let mut last_number = 0;

    for i in 0..s.len() + 1 {
        if &s[..i] == "" || &s[..i] == "-" || &s[..i] == "+" {
            continue;
        }

        match s[..i].parse::<i32>() {
            Ok(number) => last_number = number,
            Err(err) => {
                if *err.kind() == std::num::IntErrorKind::PosOverflow {
                    return i32::max_value();
                } else if *err.kind() == std::num::IntErrorKind::NegOverflow {
                    return i32::min_value();
                } else {
                    return last_number;
                }
            }
        }
    }

    last_number
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(my_atoi("-042".to_string()), -42);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(my_atoi("1337c0d3".to_string()), 1337);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(my_atoi("0-1".to_string()), 0);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(my_atoi("words and 987".to_string()), 0);
    }
}
