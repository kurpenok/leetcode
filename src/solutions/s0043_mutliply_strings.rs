struct BigUInt {
    digits: Vec<u8>,
}

impl BigUInt {
    fn from_str(s: &str) -> Self {
        let digits: Vec<u8> = s
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        BigUInt { digits }
    }

    fn multiply(&self, other: &Self) -> Self {
        let mut result = vec![0; self.digits.len() + other.digits.len()];

        for i in 0..self.digits.len() {
            let n1 = self.digits[i];
            for j in 0..other.digits.len() {
                let n2 = other.digits[j];
                let sum = n1 * n2 + result[i + j];
                result[i + j] = sum % 10;
                result[i + j + 1] += sum / 10;
            }
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        BigUInt { digits: result }
    }

    fn to_string(&self) -> String {
        self.digits
            .iter()
            .rev()
            .map(|&d| (d + b'0') as char)
            .collect()
    }
}

pub fn multiply(num1: String, num2: String) -> String {
    let biguint_1 = BigUInt::from_str(&num1);
    let biguint_2 = BigUInt::from_str(&num2);
    biguint_1.multiply(&biguint_2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(multiply("2".to_string(), "3".to_string()), "6");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(multiply("123".to_string(), "456".to_string()), "56088");
    }
}
