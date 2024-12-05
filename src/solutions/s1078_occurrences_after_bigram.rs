pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
    let mut ocurrences = Vec::new();

    let words = text.split_whitespace().collect::<Vec<&str>>();
    for i in 2..words.len() {
        let f = words[i - 2];
        let s = words[i - 1];
        let t = words[i];

        if f == first && s == second {
            ocurrences.push(t.to_string());
        }
    }

    ocurrences
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_ocurrences(
                "alice is a good girl she is a good student".to_string(),
                "a".to_string(),
                "good".to_string(),
            ),
            ["girl", "student"],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_ocurrences(
                "we will we will rock you".to_string(),
                "we".to_string(),
                "will".to_string(),
            ),
            ["we", "rock"],
        );
    }
}
