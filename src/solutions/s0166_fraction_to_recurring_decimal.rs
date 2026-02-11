use std::collections::HashMap;

pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    if numerator == 0 {
        return "0".to_string();
    }

    let sign = if (numerator < 0) ^ (denominator < 0) {
        "-"
    } else {
        ""
    };

    let num = (numerator as i64).abs();
    let den = (denominator as i64).abs();

    let integer_part = num / den;
    let mut remainder = num % den;

    let mut result = String::new();
    result.push_str(sign);
    result.push_str(&integer_part.to_string());

    if remainder == 0 {
        return result;
    }

    result.push('.');

    let mut seen_remainders: HashMap<i64, usize> = HashMap::new();

    while remainder != 0 {
        if let Some(&pos) = seen_remainders.get(&remainder) {
            result.insert(pos, '(');
            result.push(')');
            break;
        }

        seen_remainders.insert(remainder, result.len());

        remainder *= 10;
        let digit = remainder / den;
        result.push(char::from_digit(digit as u32, 10).unwrap());

        remainder %= den;
    }

    result
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn test_case_1() {
        assert_eq!(fraction_to_decimal(1, 2), "0.5");
    }

    pub fn test_case_2() {
        assert_eq!(fraction_to_decimal(2, 1), "2");
    }

    pub fn test_case_3() {
        assert_eq!(fraction_to_decimal(4, 333), "0.(012)");
    }
}
