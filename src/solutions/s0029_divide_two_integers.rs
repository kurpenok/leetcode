pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let sign = if (dividend < 0) == (divisor < 0) {
        1
    } else {
        -1
    };

    let abs_dividend = (dividend as i64).abs() as u64;
    let abs_divisor = (divisor as i64).abs() as u64;

    let mut quotient: u64 = 0;
    let mut current_dividend = abs_dividend;

    while current_dividend >= abs_divisor {
        let mut temp_divisor = abs_divisor;
        let mut shift = 0;

        while temp_divisor <= (current_dividend >> 1) {
            temp_divisor <<= 1;
            shift += 1;
        }

        quotient += 1 << shift;
        current_dividend -= temp_divisor;
    }

    let result = match sign {
        1 => quotient as i64,
        _ => -(quotient as i64),
    };

    result.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(divide(10, 3), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(divide(7, -3), -2);
    }
}
