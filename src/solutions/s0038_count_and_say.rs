pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        return "1".to_string();
    }

    if n == 2 {
        return "11".to_string();
    }

    let mut result = "11".to_string();
    for _ in 3..=n {
        let mut temp = "".to_string();
        result.push('&');

        let mut counter = 1;
        for j in 1..result.len() {
            if result.chars().nth(j) != result.chars().nth(j - 1) {
                temp.push_str(&counter.to_string());
                temp.push(result.chars().nth(j - 1).unwrap());
                counter = 1;
            } else {
                counter += 1;
            }
        }

        result = temp;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(count_and_say(4), "1211");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_and_say(1), "1");
    }
}
